

use clap::Parser;
use reqwest::Client;
use serde_json::json;
use std::{env, process::exit};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    prompt: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("OPENAI_API_KEY").unwrap_or_else(|_| {
        eprintln!("API_KEY is not set");
        exit(1)
    });
    
    let args = Args::parse();
    let prompt = args.prompt;
    let client = Client::new();

    let response: reqwest::Response = client
        .post("https://api.openai.com/v1/engines/davinci/completions")
        .json(&json!({
            "prompt": build_prompt(&prompt),
            "max_tokens": 1000,
            "temperature": 0.0,
        }))
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .await
        .unwrap();

    let response_data = response.json::<serde_json::Value>().await.unwrap();
    let response_prompt = response_data
        .get("choices")
        .and_then(|choices| choices[0].get("text"))
        .and_then(|text| text.as_str())
        .unwrap_or("No response generated.");
    println!("{}", response_prompt);

    Ok(())
}

fn build_prompt(prompt: &str) -> String {
    format!("{prompt}:\n```bash\n#!/bin/bash\n", prompt = prompt)
}
