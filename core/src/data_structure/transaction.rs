extern crate ethereum_types;
extern crate rlp;

use super::state_update::StateUpdate;
use bytes::Bytes;
use rlp::{Decodable, DecoderError, Encodable, Rlp, RlpStream};

pub struct Transaction {
    pub state_update: StateUpdate,
    pub transaction_witness: Bytes,
}

impl Encodable for Transaction {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.begin_list(2);
        s.append(&self.state_update);
        s.append(&self.transaction_witness.as_ref());
    }
}

impl Decodable for Transaction {
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        let bytes_result: Result<Vec<u8>, DecoderError> = rlp.val_at(1);
        bytes_result.map(|bytes| {
            return Transaction {
                state_update: rlp.val_at(0).unwrap(),
                transaction_witness: Bytes::from(bytes),
            };
        })
    }
}