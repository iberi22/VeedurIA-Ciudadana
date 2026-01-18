use anyhow::Result;
use dotenvy::dotenv;
use tracing::{info, warn};
use std::env;

mod obs;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    info!("Iniciando Veeduría Ciudadana Backend v0.1.0");

    let socrata_token = env::var("SOCRATA_APP_TOKEN").unwrap_or_else(|_| {
        warn!("SOCRATA_APP_TOKEN no configurado. Usando modo anónimo (limitado).");
        "ANONYMOUS".to_string()
    });

    info!("Iniciando ingestión de datos desde SECOP II...");

    // Run data ingestion
    let mut contracts = obs::ingest::run(&socrata_token).await?;


    // Initialize NLP engine
    info!("Inicializando motor de IA (Candle + BERT)...");
    let nlp_engine = obs::nlp::BertInference::new().await;

    match nlp_engine {
        Ok(engine) => {
             if let Some(first_contract) = contracts.first() {
                if let Some(obj) = &first_contract.objeto_del_contrato {
                    info!("Generando embedding para: {:.50}...", obj);
                    match engine.embed(obj) {
                        Ok(emb) => info!("Embedding generado: shape {:?}", emb.shape()),
                        Err(e) => warn!("Error al generar embedding: {}", e),
                    }
                }
             }
        },
        Err(e) => warn!("Falló la carga del modelo NLP: {}. Continuando sin IA.", e),
    }

    // Output: Generate daily_report.json
    let output_path = "../frontend/public/daily_report.json";
    info!("Guardando reporte en: {}", output_path);

    // Generate aggregated statistics & enrich contracts
    let mut total_value = 0.0;
    let mut red_flags_count = 0;
    let mut undefined_object_count = 0;
    let mut zero_value_count = 0;

    // Histogram buckets: <10M, 10M-50M, 50M-100M, 100M-500M, >500M
    let mut histogram = [0u32; 5];

    for contract in &mut contracts {
        let mut flags = Vec::new();
        let mut val = 0.0;

        // Parse value
        if let Some(val_str) = &contract.valor_del_contrato {
             if let Ok(v) = val_str.parse::<f64>() {
                 val = v;
                 total_value += val;

                 // Zero value check
                 if val == 0.0 {
                     zero_value_count += 1;
                     flags.push("Valor Cero".to_string());
                 }

                 // Histogram buckets
                 if val < 10_000_000.0 { histogram[0] += 1; }
                 else if val < 50_000_000.0 { histogram[1] += 1; }
                 else if val < 100_000_000.0 { histogram[2] += 1; }
                 else if val < 500_000_000.0 { histogram[3] += 1; }
                 else { histogram[4] += 1; }
             }
        }

        // Check object
        if let Some(obj) = &contract.objeto_del_contrato {
            let obj_lower = obj.to_lowercase();
            if obj_lower == "no definido" || obj_lower.contains("objeto a contratar") {
                undefined_object_count += 1;
                flags.push("Objeto Indefinido".to_string());
            }
        } else {
             undefined_object_count += 1;
             flags.push("Objeto Faltante".to_string());
        }

        // Set Risk Level
        let level = if flags.len() >= 2 {
            "Alto"
        } else if !flags.is_empty() {
            "Medio"
        } else {
            "Bajo"
        };

        contract.risk_level = Some(level.to_string());
        contract.red_flags = Some(flags);
    }

    red_flags_count = zero_value_count + undefined_object_count;

    let stats = serde_json::json!({
        "total_contracts": contracts.len(),
        "total_value": total_value,
        "red_flags_count": red_flags_count,
        "zero_value_count": zero_value_count,
        "undefined_object_count": undefined_object_count,
        "histogram": {
            "0-10M": histogram[0],
            "10M-50M": histogram[1],
            "50M-100M": histogram[2],
            "100M-500M": histogram[3],
            ">500M": histogram[4]
        },
        "last_updated": chrono::Utc::now().to_rfc3339()
    });

    let json_data = serde_json::to_string_pretty(&contracts)?;
    tokio::fs::write(output_path, json_data).await?;

    let stats_path = "../frontend/public/stats.json";
    info!("Guardando estadísticas en: {}", stats_path);
    tokio::fs::write(stats_path, serde_json::to_string_pretty(&stats)?).await?;

    // Hugging Face Sync (After file is written)
    let hf_token = env::var("HF_TOKEN").unwrap_or_default();
    let hf_repo = "iberi22/veeduria-secop-ii";

    if hf_token.is_empty() {
        warn!("HF_TOKEN no configurado. Saltando sincronización con Data Lake.");
    } else {
        info!("Sincronizando con HF Hub: {}...", hf_repo);
        let lake = obs::hf_hub::HFDataLake::new(hf_repo, &hf_token);
        let report_path_buf = std::path::PathBuf::from(output_path);

        // Upload daily report
        if let Err(e) = lake.upload_file(report_path_buf, "daily_report.json").await {
             warn!("Error al subir daily_report.json a HF: {}", e);
        }

        // Upload stats
        let stats_path_buf = std::path::PathBuf::from(stats_path);
        if let Err(e) = lake.upload_file(stats_path_buf, "stats.json").await {
             warn!("Error al subir stats.json a HF: {}", e);
        }
    }

    info!("Pipeline de ingestión completado.");
    Ok(())
}
