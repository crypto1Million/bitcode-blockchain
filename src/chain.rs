use crate::block::Block;
use crate::transaction::Transaction;
use crate::crypto::hash;
use chrono::Utc;

#[derive(Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    /// Initializes the blockchain with the genesis block
    pub fn new() -> Self {
        let genesis = Block::new(
            0,
            String::from("0"),             // prev_hash
            vec![],                        // no transactions
            0,                             // nonce
            hash("genesis"),               // fake hash
        );

        Blockchain {
            chain: vec![genesis],
        }
    }

    /// Returns the latest block
    pub fn get_latest_block(&self) -> &Block {
        self.chain.last().unwrap()
    }

    /// Adds a block to the chain after validation
    pub fn add_block(&mut self, block: Block) {
        if self.is_valid_new_block(&block, self.get_latest_block()) {
            self.chain.push(block);
        } else {
            eprintln!("[Blockchain] ❌ Invalid block rejected.");
        }
    }

    /// Very basic block validation
    fn is_valid_new_block(&self, new: &Block, prev: &Block) -> bool {
        if new.index != prev.index + 1 {
            return false;
        }

        if new.previous_hash != prev.hash {
            return false;
        }

        let content = format!(
            "{}{}{:?}{}",
            new.index, new.previous_hash, new.transactions, new.nonce
        );
        let computed_hash = hash(&content);

        if new.hash != computed_hash {
            return false;
        }

        true
    }

    /// Validates the entire chain (optional for sync / P2P)
    pub fn is_valid_chain(&self) -> bool {
        for i in 1..self.chain.len() {
            if !self.is_valid_new_block(&self.chain[i], &self.chain[i - 1]) {
                return false;
            }
        }
        true
    }
}
