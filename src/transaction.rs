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

}
impl Transaction {
    pub fn new_dummy(from: &str, to: &str, amount: u64, fee: u64) -> Self {
        Self {
            from: from.to_string(),
            to: to.to_string(),
            amount,
            fee,
            signature: "dummysig".into(),
            timestamp: 0,
        }
    }

    pub fn generate_signed(from: &str, to: &str, amount: u64, fee: u64) -> Self {
        // Generate dummy key and signature here (or real signing logic if test-only key is available)
        Self::new_dummy(from, to, amount, fee)
    }
}
