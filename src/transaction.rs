use serde::{Serialize, Deserialize};

/// Represents a blockchain transaction
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    /// Base64-encoded public key of sender
    pub from: String,

    /// Base64-encoded recipient address
    pub to: String,

    /// Amount of tokens to transfer
    pub amount: u64,

    /// Fee paid to miner for this TX
    pub fee: u64,

    /// Base64-encoded digital signature of TX
    pub signature: String,

    /// Timestamp for TX lifetime / expiry checks
    pub timestamp: u64,
}
