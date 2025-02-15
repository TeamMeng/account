use account::run;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    run().await?;
    Ok(())
}
