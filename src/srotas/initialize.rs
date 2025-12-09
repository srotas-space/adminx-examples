// test/src/srotas/initialize.rs

use std::sync::Arc;
use once_cell::sync::OnceCell;
use anyhow::{Result, Context};
use srotas_vector_sdk::Client;
use srotas_vector_sdk::types::CreateCollectionRequest;

#[derive(Clone)]
pub struct SrotasVectorOvm {
    pub client: Arc<srotas_vector_sdk::Client>,
}

static SROTAS_VECTOR_OVM_CONNECTION: OnceCell<Arc<SrotasVectorOvm>> = OnceCell::new();

pub struct SrotasVector;

impl SrotasVector {
    pub async fn initialize(cfg: Option<SrotasConfig>) -> Result<Arc<SrotasVectorOvm>> {
        if let Some(existing) = SROTAS_VECTOR_OVM_CONNECTION.get() {
            return Ok(existing.clone());
        }
        let cfg = cfg.unwrap_or_else(SrotasConfig::from_env);

        let client = Client::builder(&cfg.base_url, &cfg.api_key)
            .user_agent(&cfg.user_agent)
            .timeout_ms(cfg.timeout_ms)
            .retries(cfg.retries)
            .backoff_ms(cfg.backoff_ms)
            .build()
            .context("building srotas-vector SDK client")?;

        let ctx = Arc::new(SrotasVectorOvm { client: Arc::new(client) });
        SROTAS_VECTOR_OVM_CONNECTION.set(ctx.clone()).ok();
        Ok(ctx)
    }

    pub fn srotas_vector_ovm_connection() -> Arc<SrotasVectorOvm> {
        SROTAS_VECTOR_OVM_CONNECTION.get().expect("SrotasVector not initialized").clone()
    }
}

#[derive(Clone, Debug)]
pub struct SrotasConfig {
    pub base_url: String,
    pub api_key: String,
    pub user_agent: String,
    pub timeout_ms: u64,
    pub retries: u32,
    pub backoff_ms: u64,
}
impl SrotasConfig {
    pub fn from_env() -> Self {
        Self {
            base_url: std::env::var("SROTAS_BASE_URL").unwrap_or_else(|_| "http://localhost:8080".into()),
            api_key:  std::env::var("SROTAS_API_KEY").unwrap_or_else(|_| "dev-key".into()),
            user_agent: std::env::var("SROTAS_UA").unwrap_or_else(|_| "test-app/ovm".into()),
            timeout_ms: std::env::var("SROTAS_TIMEOUT_MS").ok().and_then(|s| s.parse().ok()).unwrap_or(10_000),
            retries:    std::env::var("SROTAS_RETRIES").ok().and_then(|s| s.parse().ok()).unwrap_or(1),
            backoff_ms: std::env::var("SROTAS_BACKOFF_MS").ok().and_then(|s| s.parse().ok()).unwrap_or(300),
        }
    }
}

