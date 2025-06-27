use k256::ecdsa::{SigningKey, Signature, signature::Signer, VerifyingKey, signature::Verifier};
use rand_core::OsRng;
use sha2::{Sha256, Digest};

pub struct Wallet {
    pub private_key: SigningKey,
    pub public_key: VerifyingKey,
}

impl Wallet {
    /// Generate a new wallet with ECDSA keys
    pub fn new() -> Self {
        let private_key = SigningKey::random(&mut OsRng);
        let public_key = VerifyingKey::from(&private_key);

        Wallet {
            private_key,
            public_key,
        }
    }

    /// Return address derived from the public key
    pub fn get_address(&self) -> String {
        let pubkey_bytes = self.public_key.to_encoded_point(false).as_bytes();
        let mut hasher = Sha256::new();
        hasher.update(pubkey_bytes);
        let hash = hasher.finalize();
        hex::encode(&hash[..20]) // First 20 bytes (like ETH)
    }

    /// Sign arbitrary data (e.g., transaction hash)
    pub fn sign(&self, data: &[u8]) -> Signature {
        self.private_key.sign(data)
    }

    /// Verify data using given signature and public key
    pub fn verify(public_key: &VerifyingKey, data: &[u8], sig: &Signature) -> bool {
        public_key.verify(data, sig).is_ok()
    }
}
