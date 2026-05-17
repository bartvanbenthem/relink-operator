mod crd;
mod reconciler;
mod controller;

use std::sync::Arc;
use kube::Client;
use crate::reconciler::ContextData;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    println!("Starting relink-operator process...");

    let client = Client::try_default().await?;

    let context = Arc::new(ContextData {
        client,
    });

    controller::run(context.client.clone(), context).await;

    Ok(())
}
