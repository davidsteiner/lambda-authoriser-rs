mod decode;
mod jwk;

use std::collections::HashMap;

use anyhow::{bail, Result};
use aws_lambda_events::event::apigw::{
    ApiGatewayCustomAuthorizerPolicy, ApiGatewayCustomAuthorizerRequest,
    ApiGatewayCustomAuthorizerResponse, IamPolicyStatement,
};
use serde_json::Value;
use tracing::info;

use crate::authoriser::decode::verify_claims;

pub async fn authorise(event: Value) -> Result<ApiGatewayCustomAuthorizerResponse> {
    let request = serde_json::from_value::<ApiGatewayCustomAuthorizerRequest>(event)?;
    let claims;

    let username = if let Some(token) = request.authorization_token {
        claims = verify_claims(&token).await?;
        let username = claims.get_username()?;
        info!("verified user: {:?}", username);
        username
    } else {
        bail!("no authorisation token in request");
    };

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
