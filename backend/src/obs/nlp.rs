use anyhow::{Error as E, Result};
use candle_core::{Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::bert::{BertModel, Config};
use hf_hub::{api::tokio::Api, Repo, RepoType};
use tokenizers::Tokenizer;

pub struct BertInference {
    model: BertModel,
    tokenizer: Tokenizer,
    device: Device,
}

impl BertInference {
    pub async fn new() -> Result<Self> {
        let device = Device::Cpu; // Force CPU for simplicity in GitHub Actions compatibility

        // Use a lightweight, multilingual model: sentence-transformers/paraphrase-multilingual-MiniLM-L12-v2
        // Or standard bert-base-multilingual-cased.
        // For simplicity and Candle examples, we use "sentence-transformers/all-MiniLM-L6-v2"
        // But for Spanish text, "dccuchile/bert-base-spanish-wwm-uncased" is better.
        // Let's stick to valid HF Hub weights.
        let model_id = "sentence-transformers/all-MiniLM-L6-v2".to_string();
        let revision = "main".to_string();

        let api = Api::new()?;
        let repo = api.repo(Repo::with_revision(model_id, RepoType::Model, revision));

        let config_filename = repo.get("config.json").await?;
        let tokenizer_filename = repo.get("tokenizer.json").await?;
        let weights_filename = repo.get("model.safetensors").await?;

        let config: Config = serde_json::from_str(&std::fs::read_to_string(config_filename)?)?;
        let tokenizer = Tokenizer::from_file(tokenizer_filename).map_err(E::msg)?;
        let vb = unsafe { VarBuilder::from_mmaped_safetensors(&[weights_filename], candle_core::DType::F32, &device)? };

        let model = BertModel::load(vb, &config)?;

        Ok(Self {
            model,
            tokenizer,
            device,
        })
    }

    pub fn embed(&self, text: &str) -> Result<Tensor> {
        let tokens = self.tokenizer
            .encode(text, true)
            .map_err(E::msg)?
            .get_ids()
            .to_vec();

        let token_ids = Tensor::new(&tokens[..], &self.device)?.unsqueeze(0)?;
        let token_type_ids = token_ids.zeros_like()?; // BERT uses token types, 0 for first sentence

        let embeddings = self.model.forward(&token_ids, &token_type_ids, None)?;

        // Mean pooling: average of all token embeddings (dim 1)
        let (_b_size, _seq_len, _hidden_size) = embeddings.dims3()?;
        let mean_embedding = (embeddings.sum(1)? / (_seq_len as f64))?;

        Ok(mean_embedding)
    }
}
