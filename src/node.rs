use std::sync::{Arc, Mutex};
use std::thread;
use crate::blockchain::Blockchain;
use crate::network::server::start_p2p_listener;
use crate::mempool::Mempool;
use crate::miner::start_mining;
use crate::api::start_api_server;

/// Starts a full Bitcode node
pub fn start_node() {
    println!("[Bitcode Node] Booting up...");

    // Shared blockchain and mempool
    let blockchain = Arc::new(Mutex::new(Blockchain::new()));
    let mempool = Arc::new(Mutex::new(Mempool::new()));

    // Clone for API and Miner
    let blockchain_api = Arc::clone(&blockchain);
    let mempool_api = Arc::clone(&mempool);
    let blockchain_miner = Arc::clone(&blockchain);
    let mempool_miner = Arc::clone(&mempool);

    // Start P2P WebSocket listener
    thread::spawn(move || {
        start_p2p_listener(blockchain.clone(), mempool.clone());
    });

    // Start Mining thread
    thread::spawn(move || {
        start_mining(blockchain_miner, mempool_miner);
    });

    // Start API Server (REST or WebSocket)
    start_api_server(blockchain_api, mempool_api);
}
