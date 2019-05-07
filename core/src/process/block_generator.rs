extern crate ethereum_types;

use crate::data_structure::block::Block;
use crate::data_structure::signed_transaction::SignedTransaction;
use ethereum_types::H256;

pub enum Error {}

pub struct BlockGenerator {}

impl BlockGenerator {
    pub fn generate(signed_transactions: Vec<SignedTransaction>) -> Result<Block, Error> {
        // TODO: caluculate merkle root
        return Ok(Block::new(signed_transactions, H256::zero()));
    }
}