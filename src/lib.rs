pub mod chain;
pub mod miner;
pub mod wallet;
pub mod node;
pub mod transaction;
pub mod contract;
pub mod block; // if you have it

// Optionally re-export important structs
pub use chain::Blockchain;
pub use transaction::Transaction;
pub use block::Block;
