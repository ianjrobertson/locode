use ollama_rs::Ollama;
use ollama_rs::generation::completion::request::GenerationRequest;
//use tokio::io::{self, AsyncWriteExt};
use std::io;

async fn generate_response(prompt: &str) -> String {
    let ollama = Ollama::default();

    let model = "qwen2.5-coder:14b".to_string();

    let system_prompt = "You are a helpful coding assistant. You provide clear, concise code examples and explanations.";

    let res = ollama
        .generate(
            GenerationRequest::new(model, prompt.to_string())
                .system(system_prompt.to_string())
        )
        .await;

    if let Ok(res) = res {
        return res.response;
    } else {
        return String::from("Error generating response");
    }
}

#[tokio::main]
async fn main() {
    println!("Welcome to locode, a local coding agent built with Rust!");

    loop {
        println!("Enter a command:");
        let mut input: String = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim();
        if input == "exit" {
            break;
        } else {
            println!("Cooking up a response...");
            let response = generate_response(input).await;
            println!("{}", termimad::inline(response.as_str()));
        }
    }
}
