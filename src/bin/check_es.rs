#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]

use datatect::*;
use clap::Parser;
use reqwest::blocking::Client;
use std::fs::read_to_string;
use serde_json::Value as Json;

type DynamicErrorResult<T> = Result<T, Box<dyn std::error::Error>>;

/// Simple ES Validation
#[derive(Parser, Debug)]
struct Opts {
    /// schema file to test against
    #[clap(short, long)]
    schema: String,

    /// ES host to use
    ///
    /// This must be the http root address of the ES server.  If not
    /// provided will attempt to use the env `ES_HOST` and will also
    /// attempt to find it in a .env file
    #[clap(short, long)]
    host: Option<String>,

    /// ES index to scan
    ///
    /// If not provided it will attempt to use the env `ES_INDEX` and
    /// will also attempt to find it in a .env file
    #[clap(short, long)]
    index: Option<String>,

    /// Only print errors
    #[clap(long)]
    only_errors: bool
}

impl Opts {
    fn schema(&self) -> DynamicErrorResult<Schema> {
        let data = read_to_string(&self.schema)?;
        Ok(serde_yaml::from_str::<Json>(&data)?.into())
    }

    fn documents(&self) -> DynamicErrorResult<ElasticSearchDocumentIterator<Client>> {
        let host = self
            .host
            .clone()
            .map_or_else(|| std::env::var("ES_HOST"), Ok)?;

        let index = self
            .index
            .clone()
            .map_or_else(|| std::env::var("ES_INDEX"), Ok)?;

        Ok(ElasticSearchDocumentIterator::new(host, index))
    }
}

fn main() -> DynamicErrorResult<()> {
    dotenv::dotenv().ok();
    let opts = Opts::parse();

    let schema = opts.schema()?;
    let mut count = 0;
    let mut fails = vec![];

    for data in opts.documents()? {
        count += 1;

        if ! schema.is_valid(&data["_source"]) {
            fails.push(data["_id"].as_str().unwrap().to_owned());
        }

        if count % 1000 == 0 {
            println!("Validated: {count}.  Failures: {}", fails.len());
        }
    }

    println!("Validated {count} documents.");

    if ! fails.is_empty() {
        println!("Failures:");
        for fail in fails {
            println!("  {fail}");
        }
    }

    Ok(())
}
