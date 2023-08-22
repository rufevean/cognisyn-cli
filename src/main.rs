
use clap::Parser;
use reqwest::Client;
use serde_json::json;
use std::{env, process::exit};
use serde_json::Value;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    prompt: Vec<String>,
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
    let api_base = env::var("OPENAI_API_BASE")
            .unwrap_or_else(|_| String::from("https://api.openai.com/v1"));
 
    let api_addr = format!("{}/completions", api_base);


let response = client
    .post(api_addr)
    .json(&json!({
        "top_p": 1,
        "stop": "```",
        "temperature": 0,
        "suffix": "\n```",
        "max_tokens": 1000,
        "presence_penalty": 0,
        "frequency_penalty": 0,
        "model": "text-davinci-003",
        "prompt": build_prompt(&prompt.join(" ")),
    }))
    .header("Authorization", format!("Bearer {}", api_key))
    .send()
    .await
    .unwrap();

// Extract and print the status code
    let response_json: Value = response.json().await?;

    // Extract the text from the JSON
    if let Some(choices) = response_json.get("choices").and_then(|choices| choices.as_array()) {
        if let Some(choice) = choices.first() {
            if let Some(text) = choice.get("text").and_then(|text| text.as_str()) {
                println!("{}"text);
            }
        }
    }
    Ok(())
}

fn build_prompt(prompt: &str) -> String {
    format!("{prompt}:\n```bash\n#!/bin/bash\n", prompt = prompt)
}
