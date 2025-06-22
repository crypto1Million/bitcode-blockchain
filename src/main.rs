mod chain;
use chain::*;
use std::sync::{Arc, Mutex};

fn main() {
    let state = Arc::new(Mutex::new(Blockchain::new()));
    
    // Serve block explorer
    serve_block_explorer_api(state.clone(), 8080);

    // Start WebSocket server
    start_websocket_server(6000);

    // Inject browser wallet
    inject_browser_wallet_script("public");

    // Mine a faucet block to demo
    let mut bc = state.lock().unwrap();
    mine_faucet_block("test_pubkey_address", &mut bc);

    println!("🚀 Bitcode node initialized. Open http://localhost:8080/explorer");
}
