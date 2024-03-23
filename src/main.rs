use anyhow::Result;
use clap::Parser;

use httpie_rs::args::{Args, Commands};
use httpie_rs::http::{get, post};
use reqwest::{header, Client};

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let mut headers = header::HeaderMap::new();
    headers.insert("X-POWERED-BY", "Rust".parse()?);
    headers.insert(header::USER_AGENT, "Rust Httpie".parse()?);

    let client = Client::builder().default_headers(headers).build()?;

    let result = match args.command {
        Commands::Get(data) => get(client, &data).await?,
        Commands::Post(data) => post(client, data).await?,
    };

    Ok(result)
}
