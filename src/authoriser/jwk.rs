use anyhow::{bail, Result};
use serde::Deserialize;
use serde_json::Value;
use tokio::sync::OnceCell;
use tracing::info;

static CACHED_KEYS: OnceCell<Vec<Jwk>> = OnceCell::const_new();

#[derive(Debug, Deserialize)]
pub struct Jwk {
    pub kid: String,
    pub e: String,
    pub n: String,
}

pub async fn keys() -> Result<&'static Vec<Jwk>> {
    Ok(CACHED_KEYS.get_or_try_init(fetch_keys).await?)
}

async fn fetch_keys() -> Result<Vec<Jwk>> {
    info!("fetching jwk");
    let client = reqwest::Client::builder().use_rustls_tls().build()?;

    let url = std::env::var("JWKS_URL")?;
    let res = client.get(url).send().await?;

    let jwk_text = res.text().await?;

    let keys_value = match serde_json::from_str::<Value>(&jwk_text)? {
        serde_json::Value::Object(mut obj) => match obj.get_mut("keys") {
            Some(val) => val.take(),
            None => bail!("no keys found in JWK JSON"),
        },
        _ => bail!("JWK is not a mapping for keys"),
    };

    let keys: Vec<Jwk> = serde_json::from_value(keys_value)?;
    Ok(keys)
}
