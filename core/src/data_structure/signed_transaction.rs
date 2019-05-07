extern crate ethereum_types;
extern crate rlp;

use super::state_object::StateObject;
use super::state_update::StateUpdate;
use super::transaction::Transaction;
use rlp::{Decodable, DecoderError, Encodable, Rlp, RlpStream};

pub struct SignedTransaction {
    pub transactions: Vec<Transaction>,
}

impl Encodable for SignedTransaction {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.append_list(self.transactions.as_slice());
    }
}

impl Decodable for SignedTransaction {
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        if (!rlp.is_list()) {
            return Err(DecoderError::Custom("Provided byte data isn't RLP list."));
        }
        let transactions_result: Result<Vec<Transaction>, DecoderError> = rlp.as_list();
        return transactions_result.map(|list| {
            return SignedTransaction { transactions: list };
        });
    }
}

#[cfg(test)]
mod tests {
    use super::DecoderError;
    use super::SignedTransaction;
    use super::StateObject;
    use super::StateUpdate;
    use super::Transaction;
    use bytes::Bytes;
    use ethereum_types::Address;

    #[test]
    fn test_rlp_encode() {
        let message = "parameters".as_bytes();
        let message_bytes = Bytes::from(message);
        let witness = "witness".as_bytes();
        let witness_bytes = Bytes::from(witness);
        let state_object = StateObject {
            predicate: Address::zero(),
            parameters: message_bytes,
        };
        let state_update = StateUpdate {
            start: 0,
            end: 0,
            block: 0,
            plasma_contract: Address::zero(),
            new_state: state_object,
        };
        let transaction = Transaction {
            state_update: state_update,
            transaction_witness: witness_bytes,
        };
        let _signed_transaction = SignedTransaction {
            transactions: vec![transaction],
        };
        let encoded = rlp::encode(&_signed_transaction);
        let _decoded: SignedTransaction = rlp::decode(&encoded).unwrap();
        assert_eq!(
            _decoded.transactions.len(),
            _signed_transaction.transactions.len()
        );
    }

    #[test]
    fn fail_to_decode() {
        let animal = "failtodecode";
        let encoded = rlp::encode(&animal);
        let decoded: Result<SignedTransaction, DecoderError> = rlp::decode(&encoded);
        assert_eq!(decoded.is_err(), true);
    }
}