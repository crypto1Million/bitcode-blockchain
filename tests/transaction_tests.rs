use bitcode_blockchain::{Transaction, Blockchain};

#[test]
fn test_transaction_signature() {
    let tx = Transaction::generate_signed("alice", "bob", 100, 1);
    let chain = Blockchain::new();

    assert!(chain.verify_transaction_signature(&tx), "Transaction signature should be valid");
}

#[test]
fn test_invalid_signature() {
    let mut tx = Transaction::generate_signed("alice", "bob", 100, 1);
    tx.signature = "invalidsig".into();

    let chain = Blockchain::new();
    assert!(!chain.verify_transaction_signature(&tx), "Tampered signature should be invalid");
}
