use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub timestamp: u128,
    pub previous_hash: String,
    pub nonce: u64,
    pub data: String,
    pub hash: String,
}

impl Block {
    /// Create the genesis block
    pub fn genesis() -> Self {
        let mut block = Block {
            index: 0,
            timestamp: now(),
            previous_hash: "0".repeat(64),
            nonce: 0,
            data: "Genesis Block".to_string(),
            hash: String::new(),
        };
        block.hash = block.calculate_hash();
        block
    }

    /// Create a new block with given data and previous hash
    pub fn new(index: u64, previous_hash: String, data: String) -> Self {
        let mut block = Block {
            index,
            timestamp: now(),
            previous_hash,
            nonce: 0,
            data,
            hash: String::new(),
        };
        block.mine_block(2); // adjustable difficulty
        block
    }

    /// Calculate SHA256 hash of the block
    pub fn calculate_hash(&self) -> String {
        let block_string = format!(
            "{}{}{}{}{}",
            self.index,
            self.timestamp,
            self.previous_hash,
            self.nonce,
            self.data
        );

        let mut hasher = Sha256::new();
        hasher.update(block_string.as_bytes());
        let result = hasher.finalize();
        hex::encode(result)
    }

    /// Proof of Work: brute-force a hash that starts with N zeros
    pub fn mine_block(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);
        loop {
            self.hash = self.calculate_hash();
            if &self.hash[..difficulty] == target {
                break;
            }
            self.nonce += 1;
        }
        println!("Block mined: {}", self.hash);
    }
}

/// Get current timestamp in milliseconds
fn now() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}
