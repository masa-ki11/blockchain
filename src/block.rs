use serde::{Serialize, Deserialize};
use crypto_hash::{Algorithm, hex_digest};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    index: u32,
    timestamp: i64,
    data: String,
    prev_hash: String,
    hash: String,
}

impl Block {
    pub fn new(index: u32, timestamp: i64, data: String, prev_hash: String) -> Block {
        let mut block = Block {
            index,
            timestamp,
            data,
            prev_hash,
            hash: "".to_string(),
        };
        block.hash_block();
        block
    }

    pub fn calculate_hash(&self) -> String {
            let input = format!("{}{}{}{}", &self.index, &self.timestamp, &self.data, &self.prev_hash);
            hex_digest(Algorithm::SHA256, input.as_bytes())
        }

    fn hash_block(&mut self) {
        let input = format!("{}{}{}{}", &self.index, &self.timestamp, &self.data, &self.prev_hash);
        self.hash = hex_digest(Algorithm::SHA256, input.as_bytes());
    }

    pub fn prev_hash(&self) -> &String {
            &self.prev_hash
        }

    pub fn hash(&self) -> &String {
        &self.hash
    }

    pub fn index(&self) -> u32 {
        self.index
    }

    pub fn timestamp(&self) -> i64 {
        self.timestamp
    }

    pub fn data(&self) -> &String {
        &self.data
    }
}
