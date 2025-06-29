use bitcode_blockchain::{Blockchain, Block};

#[test]
fn test_chain_validity_check() {
    let mut chain = Blockchain::new();
    let tx = Transaction::new_dummy("alice", "bob", 50, 1);
    chain.pending_transactions.push(tx);
    chain.mine_block();

    assert!(chain.is_valid_chain(), "Blockchain should be valid after mining");
}

#[test]
fn test_invalid_block_hash() {
    let mut chain = Blockchain::new();
    chain.mine_block();
    chain.blocks[1].hash = "tampered".into();

    assert!(!chain.is_valid_chain(), "Blockchain with tampered hash should be invalid");
}
