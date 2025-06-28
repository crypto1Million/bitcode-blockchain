use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use crate::blockchain::{Block, Blockchain};
use crate::mempool::Mempool;
use crate::transaction::Transaction;
use crate::crypto::hash;
use crate::block::Block;

const DIFFICULTY_PREFIX: &str = "0000"; // Adjustable

/// Start mining loop (background thread)
pub fn start_mining(blockchain: Arc<Mutex<Blockchain>>, mempool: Arc<Mutex<Mempool>>) {
    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(5)); // simulate block time

        let txs = {
            let mut pool = mempool.lock().unwrap();
            pool.drain_pending_transactions(5) // Get 5 txs from mempool
        };

        if txs.is_empty() {
            println!("[Miner] No transactions to mine...");
            continue;
        }

        let mut chain = blockchain.lock().unwrap();
        let last_block = chain.get_latest_block();
        let index = last_block.index + 1;
        let previous_hash = last_block.hash.clone();

        println!("[Miner] Mining block #{} with {} txs...", index, txs.len());

        let (nonce, hash) = mine_block(index, &previous_hash, &txs);
        let new_block = Block::new(index, previous_hash, txs, nonce, hash);

        chain.add_block(new_block);
        println!("[Miner] 🟩 Block #{} mined and added to chain!", index);
    });
}

/// Simple PoW mining loop
fn mine_block(index: u64, prev_hash: &str, txs: &[Transaction]) -> (u64, String) {
    let mut nonce = 0;

    loop {
        let content = format!("{}{}{:?}{}", index, prev_hash, txs, nonce);
        let hash_result = hash(&content);

        if hash_result.starts_with(DIFFICULTY_PREFIX) {
            return (nonce, hash_result);
        }

        nonce += 1;
    }
}
