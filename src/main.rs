use anyhow::Result;
use tracing_subscriber::FmtSubscriber;

mod comicat;
mod config;
mod prop;

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = FmtSubscriber::builder().finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let config = config::parse();

    let one = comicat::ComiCat::new(&config.kwords);
    let res = one.find().await?;
    Ok(res)
}
