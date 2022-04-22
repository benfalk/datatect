#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]

use clap::Parser;
use datatect::*;
use serde_json::Value as Json;
use std::fs::read_to_string;

/// Simple Schema Validation
#[derive(Parser, Debug)]
struct Opts {
    /// schema file to test against
    #[clap(short)]
    schema: String,
    files: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = Opts::parse();
    let schema: Schema = serde_yaml::from_str::<Json>(&read_to_string(&opts.schema)?)?.into();

    for file in &opts.files {
        let data = serde_json::from_str(&read_to_string(file)?)?;

        if let Err(errors) = schema.validate(&data) {
            println!("ERROR: {file}");
            for error in errors {
                println!("  {} at {}", error, error.instance_path);
            }
        }
        else {
            println!("PASSED: {file}");
        };
    }

    Ok(())
}
