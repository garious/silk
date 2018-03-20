//! The `signature` crate provides functionality for public and private keys

use generic_array::GenericArray;
use generic_array::typenum::{U32, U64};
use ring::signature::Ed25519KeyPair;
use ring::{rand, signature};
use untrusted;
use base64;
use serde::{Deserialize, Deserializer, Serializer};

pub fn as_base64<T, S>(key: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    T: AsRef<[u8]>,
    S: Serializer,
{
    serializer.serialize_str(&base64::encode(key.as_ref()))
}

pub fn from_base64<'de, D>(deserializer: D) -> Result<GenericArray<u8, U32>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;
    String::deserialize(deserializer)
        .and_then(|string| base64::decode(&string).map_err(|err| Error::custom(err.to_string())))
        .map(|bytes| GenericArray::clone_from_slice(&bytes))
}

pub fn from_base64_u64<'de, D>(deserializer: D) -> Result<GenericArray<u8, U64>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;
    String::deserialize(deserializer)
        .and_then(|string| base64::decode(&string).map_err(|err| Error::custom(err.to_string())))
        .map(|bytes| GenericArray::clone_from_slice(&bytes))
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Copy, Clone, Hash, Default)]
pub struct PublicKey(
    #[serde(serialize_with = "as_base64", deserialize_with = "from_base64")] GenericArray<u8, U32>,
);

impl AsRef<GenericArray<u8, U32>> for PublicKey {
    fn as_ref(&self) -> &GenericArray<u8, U32> {
        &self.0
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Copy, Clone, Hash, Default)]
pub struct Signature(
    #[serde(serialize_with = "as_base64", deserialize_with = "from_base64_u64")]
    GenericArray<u8, U64>,
);

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

pub struct KeyPair(Ed25519KeyPair);

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
        PublicKey(GenericArray::clone_from_slice(self.0.public_key_bytes()))
    }

    pub fn sign(&self, msg: &[u8]) -> Signature {
        Signature(GenericArray::clone_from_slice(self.0.sign(msg).as_ref()))
    }
}
