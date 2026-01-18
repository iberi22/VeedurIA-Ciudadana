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
    let contracts = obs::ingest::run(&socrata_token).await?;


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

    let json_data = serde_json::to_string_pretty(&contracts)?;
    tokio::fs::write(output_path, json_data).await?;

    // Generate aggregated statistics
    let mut total_value = 0.0;
    let mut red_flags_count = 0;
    let mut undefined_object_count = 0;
    let mut zero_value_count = 0;

    for contract in &contracts {
        // Parse value
        if let Some(val_str) = &contract.valor_del_contrato {
             if let Ok(val) = val_str.parse::<f64>() {
                 total_value += val;
                 if val == 0.0 {
                     zero_value_count += 1;
                 }
             }
        }

        // Check object
        if let Some(obj) = &contract.objeto_del_contrato {
            if obj == "No definido" || obj == "Objeto a contratar" {
                undefined_object_count += 1;
            }
        } else {
             undefined_object_count += 1;
        }
    }

    // Simple heuristic for total red flags
    red_flags_count = zero_value_count + undefined_object_count;

    let stats = serde_json::json!({
        "total_contracts": contracts.len(),
        "total_value": total_value,
        "red_flags_count": red_flags_count,
        "zero_value_count": zero_value_count,
        "undefined_object_count": undefined_object_count,
        "last_updated": chrono::Utc::now().to_rfc3339()
    });

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
