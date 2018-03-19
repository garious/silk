//! The `event` crate provides the data structures for log events.

use signature::{KeyPair, PublicKey, Signature};
use transaction::Transaction;
use chrono::prelude::*;
use bincode::serialize;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum Event {
    Transaction(Transaction),
    Signature {
        from: PublicKey,
        tx_sig: Signature,
        sig: Signature,
    },
    Timestamp {
        from: PublicKey,
        dt: DateTime<Utc>,
        sig: Signature,
    },
}

impl Event {
    pub fn new_timestamp(from: &KeyPair, dt: DateTime<Utc>) -> Self {
        let sign_data = serialize(&dt).unwrap();
        let sig = from.sign(&sign_data);
        Event::Timestamp {
            from: from.pubkey(),
            dt,
            sig,
        }
    }

    // TODO: Rename this to transaction_signature().
    pub fn get_signature(&self) -> Option<Signature> {
        match *self {
            Event::Transaction(ref tr) => Some(tr.sig),
            Event::Signature { .. } => None,
            Event::Timestamp { .. } => None,
        }
    }

    pub fn verify(&self) -> bool {
        match *self {
            Event::Transaction(ref tr) => tr.verify(),
            Event::Signature { from, tx_sig, sig } => sig.verify(from.as_ref(), tx_sig.as_ref()),
            Event::Timestamp { from, dt, sig } => {
                sig.verify(from.as_ref(), &serialize(&dt).unwrap())
            }
        }
    }
}
