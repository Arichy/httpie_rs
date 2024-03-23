use std::str::FromStr;

use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};
use reqwest::Url;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Get(Get),
    Post(Post),
}

#[derive(clap::Args, Debug)]
pub struct Get {
    pub url: String,
}

#[derive(clap::Args, Debug)]
pub struct Post {
    #[arg(value_parser = parse_url)]
    pub url: String,

    #[arg(value_parser = parse_kv_pair)]
    pub body: Vec<KvPair>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct KvPair {
    pub k: String,
    pub v: String,
}

impl FromStr for KvPair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split("=");
        let err = || anyhow!(format!("Failed to parse{}", s));
        Ok(Self {
            k: (split.next().ok_or_else(err)?).to_string(),
            v: (split.next().ok_or_else(err)?).to_string(),
        })
    }
}

fn parse_kv_pair(s: &str) -> Result<KvPair> {
    s.parse()
}

fn parse_url(s: &str) -> Result<String> {
    let _url: Url = s.parse()?;
    Ok(s.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_url_works() {
        assert!(parse_url("https://google.com").is_ok());
        assert!(parse_url("abc").is_err());
        assert!(parse_url("https://httpbin.org/post").is_ok());
    }

    #[test]
    fn parse_kv_pairs_works() {
        assert!(parse_kv_pair("a").is_err());
        assert_eq!(
            parse_kv_pair("a=1").unwrap(),
            KvPair {
                k: "a".into(),
                v: "1".into(),
            }
        );
    }
}
