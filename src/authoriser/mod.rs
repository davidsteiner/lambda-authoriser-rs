mod decode;
mod jwk;

use std::collections::HashMap;

use anyhow::Result;
use aws_lambda_events::event::apigw::{
    ApiGatewayCustomAuthorizerPolicy, ApiGatewayCustomAuthorizerResponse, IamPolicyStatement,
};
use serde::Deserialize;
use serde_json::Value;
use tracing::info;

use crate::authoriser::decode::verify_claims;

#[derive(Deserialize, Debug)]
struct AuthHeaders {
    #[serde(rename = "Authorization")]
    authorization: String,
}

#[derive(Deserialize, Debug)]
struct AuthorisationRequestEvent {
    headers: AuthHeaders,
}

pub async fn authorise(event: Value) -> Result<ApiGatewayCustomAuthorizerResponse> {
    info!("authorisation event: {:?}", event);
    let request = serde_json::from_value::<AuthorisationRequestEvent>(event)?;

    let token = request.headers.authorization;
    let claims = verify_claims(&token).await?;
    let username = claims.username;

    let mut context = HashMap::new();
    context.insert("username".to_owned(), serde_json::to_value(username)?);

    let stmt = IamPolicyStatement {
        action: vec!["execute-api:Invoke".to_owned()],
        effect: Some("Allow".to_owned()),
        resource: vec!["*".to_owned()],
    };

    let response = ApiGatewayCustomAuthorizerResponse {
        principal_id: None,
        policy_document: ApiGatewayCustomAuthorizerPolicy {
            version: Some("2012-10-17".to_owned()),
            statement: vec![stmt],
        },
        context,
        usage_identifier_key: None,
    };

    Ok(response)
}
