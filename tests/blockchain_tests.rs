use bitcode_blockchain::{Blockchain, Transaction, Block};

#[test]
fn test_blockchain_initialization() {
    let chain = Blockchain::new();
    assert_eq!(chain.blocks.len(), 1, "Blockchain should start with genesis block");
}

#[test]
fn test_block_addition() {
    let mut chain = Blockchain::new();
    let tx = Transaction::new_dummy("alice", "bob", 100, 1);
    chain.pending_transactions.push(tx);
    chain.mine_block();

    assert_eq!(chain.blocks.len(), 2, "A block should be added after mining");
}
