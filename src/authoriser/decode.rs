use anyhow::{bail, Result};
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use serde::Deserialize;
use tracing::info;

use crate::authoriser::jwk;

#[derive(Debug, Deserialize)]
pub struct Claims {
    #[serde(rename = "cognito:username")]
    cognito_username: String,
    identities: Vec<Identity>,
}

#[derive(Debug, Deserialize)]
pub struct Identity {
    #[serde(rename = "userId")]
    user_id: String,
}

impl Claims {
    pub fn get_username(&self) -> Result<&str> {
        if let Some(identity) = self.identities.get(0) {
            Ok(&identity.user_id)
        } else {
            bail!("no identities in claims")
        }
    }
}

pub async fn verify_claims(token: &str) -> Result<Claims> {
    let keys = jwk::keys().await?;

    let header = decode_header(token)?;
    let kid = match header.kid {
        Some(k) => k,
        None => bail!("token header has no kid"),
    };
    let key = match keys.iter().find(|&k| k.kid == kid) {
        Some(key) => key,
        None => bail!("none of the keys match token kid"),
    };

    info!("found appropriate key: {:?}", key);

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_rsa_components(&key.n, &key.e),
        &Validation::new(Algorithm::RS256),
    )?;

    Ok(token_data.claims)
}
