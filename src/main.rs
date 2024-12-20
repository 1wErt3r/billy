use bitcoincore_rpc::{Auth, Client, RpcApi};
use dotenv::dotenv;
use std::env;
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    
    let rpc_user = env::var("BITCOIN_RPC_USER")
        .expect("BITCOIN_RPC_USER must be set");
    let rpc_pass = env::var("BITCOIN_RPC_PASS")
        .expect("BITCOIN_RPC_PASS must be set");

    let rpc = Client::new(
        "http://127.0.0.1:8332",
        Auth::UserPass(rpc_user, rpc_pass)
    )?;

    // Create a new wallet with Taproot support
    let wallet_name = "taproot_wallet";
    let wallet_options = json!({
        "descriptors": true,
        "blank": true,
        "private_keys": true
    });

    match rpc.create_wallet(wallet_name, Some(&wallet_options)) {
        Ok(_) => println!("Created new Taproot-capable wallet: {}", wallet_name),
        Err(e) => {
            if e.to_string().contains("Database already exists") {
                println!("Wallet '{}' already exists, continuing...", wallet_name);
            } else {
                return Err(e.into());
            }
        }
    }

    // Load the wallet
    let wallet_client = Client::new(
        "http://127.0.0.1:8332/wallet/taproot_wallet",
        Auth::UserPass(rpc_user, rpc_pass)
    )?;

    // Generate a new Taproot address
    let address = wallet_client.get_new_address(None, Some(bitcoincore_rpc::json::AddressType::Bech32m))?;
    println!("Generated new Taproot address: {}", address);

    Ok(())
} 