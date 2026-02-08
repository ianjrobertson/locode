use ollama_rs::Ollama;
use ollama_rs::generation::completion::request::GenerationRequest;
//use tokio::io::{self, AsyncWriteExt};
use std::io;

fn read_file(path: String) -> Result<String, Box<dyn std::error::Error + Sync + Send>>{
    let contents = std::fs::read_to_string(path).expect("Failed to read file");
    return Ok(contents);
}

fn write_file(path: String, contents: String) -> Result<(), Box<dyn std::error::Error + Sync + Send>> {
    std::fs::write(path, contents).expect("Failed to write file");
    return Ok(());
}

fn edit_file(path: String, old_content: String, new_content: String) -> Result<(), Box<dyn std::error::Error + Sync + Send>> {
    return Ok(());
}

fn list_files() -> Result<Vec<String>, Box<dyn std::error::Error + Sync + Send>> {
    let files = std::fs::read_dir(".").expect("Failed to read directory");
    let mut file_names = Vec::new();
    for file in files {
        file_names.push(file.unwrap().path().display().to_string());
    }
    return Ok(file_names);
}

fn search_code(query: String) -> Result<Vec<String>, Box<dyn std::error::Error + Sync + Send>> {
 return Ok(Vec::new());
}

fn executre_command() -> Result<Vec<String>, Box<dyn std::error::Error + Sync + Send>> {
    return Ok(Vec::new());
}

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
