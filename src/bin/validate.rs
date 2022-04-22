#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]

use clap::Parser;
use datatect::*;
use serde_json::Value as Json;
use std::fs::read_to_string;
use rayon::prelude::*;

/// Simple Schema Validation
#[derive(Parser, Debug)]
struct Opts {
    /// schema file to test against
    #[clap(short)]
    schema: String,
    /// All of the files to validate
    files: Vec<String>,
    /// Only print errors
    #[clap(long)]
    only_errors: bool
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = Opts::parse();
    let schema: Schema = serde_yaml::from_str::<Json>(&read_to_string(&opts.schema)?)?.into();

    opts.files.par_iter().for_each(|file|{
        let file_data = read_to_string(file).unwrap();
        let json_data = serde_json::from_str(&file_data).unwrap();

        if let Err(errors) = schema.validate(&json_data) {
            println!("FAIL: {file}");
            for error in errors {
                println!("  {} at {}", error, error.instance_path);
            }
        } else {
            if ! opts.only_errors {
                println!("PASS: {file}");
            }
        };
    });

    Ok(())
}
