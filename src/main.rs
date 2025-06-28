// === Module Declarations ===
mod chain;
mod node;
mod miner;
mod block;
mod wallet;
mod contract;

// === External Crates ===
use std::sync::{Arc, Mutex};
use crate::chain::Blockchain;

fn main() {
    println!("\n🚀 Starting Bitcode Blockchain Node...\n");

    // Create initial blockchain
    let chain = Blockchain::new();
    let chain = Arc::new(Mutex::new(chain));

    // Start Web Dashboard on port 8081
    chain::start_web_dashboard(Arc::clone(&chain), 8081);

    // Start P2P network listener
    {
        let chain_clone = Arc::clone(&chain);
        std::thread::spawn(move || {
            let mut locked = chain_clone.lock().unwrap();
            locked.start_p2p_server(6000); // change port if needed
        });
    }

    // Optional: Faucet miner loop (in dev/testnet mode)
    {
        let chain_clone = Arc::clone(&chain);
        std::thread::spawn(move || {
            loop {
                {
                    let mut locked = chain_clone.lock().unwrap();
                    locked.mine_block(); // auto-mine
                }
                std::thread::sleep(std::time::Duration::from_secs(30)); // Adjustable
            }
        });
    }

    // Wallet CLI (optional interactive CLI)
    // Uncomment to launch:
    // wallet::run_wallet_cli();

    // Keep main thread alive
    loop {
        std::thread::sleep(std::time::Duration::from_secs(3600));
    }
}
