use bitcoincore_rpc::{Auth, Client, RpcApi};
use dotenv::dotenv;
use std::env;
use std::io::{self, Write};

mod ollama;

#[tokio::main]
async fn main() {
    dotenv().ok();
    
    let rpc_user = env::var("BITCOIN_RPC_USER").expect("BITCOIN_RPC_USER must be set");
    let rpc_pass = env::var("BITCOIN_RPC_PASS").expect("BITCOIN_RPC_PASS must be set");
    let ollama_server = env::var("OLLAMA_SERVER").unwrap_or_else(|_| "http://localhost:11434".to_string());
    let ollama_model = env::var("OLLAMA_MODEL").unwrap_or_else(|_| "qwen2.5".to_string());

    let rpc = match Client::new("http://127.0.0.1:8332", Auth::UserPass(rpc_user, rpc_pass)) {
        Ok(client) => client,
        Err(e) => {
            eprintln!("Failed to create RPC client: {}", e);
            std::process::exit(1);
        }
    };

    println!("Bitcoin Node Chat Interface");
    println!("Type 'exit' to quit");
    println!("Ask questions about your node...\n");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.eq_ignore_ascii_case("exit") {
            break;
        }

        match ollama::query_ollama(input, &ollama_server, &ollama_model, ollama::QueryMode::Command).await {
            Ok(rpc_command) => {
                println!("Executing command: {}", rpc_command);
                match rpc.call::<serde_json::Value>(&rpc_command, &[]) {
                    Ok(response) => {
                        println!("\nResponse:");
                        match rpc_command.as_str() {
                            "getblockcount" => println!("Current block height: {}", response),
                            _ => {
                                println!("{}", serde_json::to_string_pretty(&response).unwrap());
                                
                                // Get a summary of the JSON response
                                match ollama::query_ollama(
                                    &serde_json::to_string_pretty(&response).unwrap(),
                                    &ollama_server,
                                    &ollama_model,
                                    ollama::QueryMode::Summarize
                                ).await {
                                    Ok(summary) => println!("\nSummary:\n{}", summary),
                                    Err(e) => println!("Error getting summary: {}", e),
                                }
                            }
                        }
                    },
                    Err(e) => println!("Error executing RPC command: {}", e),
                }
            },
            Err(e) => println!("Error querying Ollama: {}", e),
        }
        println!();
    }
} 