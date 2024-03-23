use std::collections::HashMap;

use crate::{
    args::{Get, Post},
    print::print_resp,
};
use anyhow::Result;
use reqwest::Client;

pub async fn get(client: Client, args: &Get) -> Result<()> {
    let resp = client.get(&args.url).send().await?;

    Ok(print_resp(resp).await?)
}

pub async fn post(client: Client, args: Post) -> Result<()> {
    let mut body = HashMap::new();
    for pair in args.body.iter() {
        body.insert(&pair.k, &pair.v);
    }

    let resp = client.post(args.url).json(&body).send().await?;

    Ok(print_resp(resp).await?)
}
