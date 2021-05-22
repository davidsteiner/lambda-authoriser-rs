mod authoriser;

use lambda_runtime::{handler_fn, Context, Error};
use serde_json::Value;
use tracing::{info, Level};

#[tokio::main]
#[tracing::instrument]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .json()
        .init();
    let handler = handler_fn(func);
    lambda_runtime::run(handler).await?;
    Ok(())
}

async fn func(event: Value, _: Context) -> Result<Value, Error> {
    match authoriser::authorise(event).await {
        Ok(res) => Ok(serde_json::to_value(res)?),
        Err(err) => {
            info!("authorisation failed: {:?}", err);
            Err(Box::new(simple_error::SimpleError::new("Unauthorized")))
        }
    }
}
