//! The `signature` crate provides functionality for public and private keys

use generic_array::GenericArray;
use generic_array::typenum::{U32, U64};
use ring::signature::Ed25519KeyPair;
use ring::{rand, signature};
use untrusted;

pub struct KeyPair(Ed25519KeyPair);
pub type PublicKey = GenericArray<u8, U32>;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Copy, Clone, Hash, Default)]
pub struct Signature(GenericArray<u8, U64>);

impl KeyPair {
    /// Return a new ED25519 keypair
    pub fn from_pkcs8(pkcs8_bytes: &[u8]) -> Self {
        KeyPair(signature::Ed25519KeyPair::from_pkcs8(untrusted::Input::from(pkcs8_bytes)).unwrap())
    }

    /// Return a new ED25519 keypair
    pub fn new() -> Self {
        let rng = rand::SystemRandom::new();
        let pkcs8_bytes = signature::Ed25519KeyPair::generate_pkcs8(&rng).unwrap();
        Self::from_pkcs8(&pkcs8_bytes)
    }

    /// Return the public key for the given keypair
    pub fn pubkey(&self) -> PublicKey {
        GenericArray::clone_from_slice(self.0.public_key_bytes())
    }

    pub fn sign(&self, msg: &[u8]) -> Signature {
        Signature(GenericArray::clone_from_slice(self.0.sign(msg).as_ref()))
    }
}

impl Signature {
    pub fn verify(&self, peer_public_key_bytes: &[u8], msg_bytes: &[u8]) -> bool {
        let peer_public_key = untrusted::Input::from(peer_public_key_bytes);
        let msg = untrusted::Input::from(msg_bytes);
        let sig = untrusted::Input::from(&self.0);
        signature::verify(&signature::ED25519, peer_public_key, msg, sig).is_ok()
    }
}

impl AsRef<GenericArray<u8, U64>> for Signature {
    fn as_ref(&self) -> &GenericArray<u8, U64> {
        &self.0
    }
}
