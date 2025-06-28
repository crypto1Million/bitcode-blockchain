use serde::{Serialize, Deserialize};
use sha2::{Digest, Sha256};

use crate::chain::Transaction;

/// Represents a single block in the blockchain
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
    pub prev_hash: String,
    pub nonce: u64,
    pub hash: String,
}

impl Block {
    /// Create a new block and mine it (with proof-of-work)
    pub fn new(index: u64, prev_hash: String, transactions: Vec<Transaction>) -> Self {
        let timestamp = Self::current_timestamp();
        let mut nonce = 0;
        let mut hash;

        loop {
            hash = Self::calculate_hash(index, timestamp, &transactions, &prev_hash, nonce);
            if hash.starts_with("0000") {
                break;
            }
            nonce += 1;
        }

        Block {
            index,
            timestamp,
            transactions,
            prev_hash,
            nonce,
            hash,
        }
    }

    /// Calculates the SHA256 hash of the block’s contents
    pub fn calculate_hash(index: u64, timestamp: u64, transactions: &Vec<Transaction>, prev_hash: &str, nonce: u64) -> String {
        let tx_data = serde_json::to_string(transactions).unwrap_or_default();
        let data = format!("{}{}{}{}{}", index, timestamp, tx_data, prev_hash, nonce);
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }

    /// Helper to get current UNIX timestamp
    fn current_timestamp() -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
    }
}
