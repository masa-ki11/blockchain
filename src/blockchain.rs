use super::block::Block;
use chrono::prelude::*;

#[derive(Debug, Clone)]
pub struct Blockchain {
    chain: Vec<Block>,
    current_index: u32,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        let mut chain = Blockchain {
            chain: Vec::new(),
            current_index: 1,
        };
        chain.add_genesis_block();
        chain
    }

    fn add_genesis_block(&mut self) {
        let genesis_block = Block::new(0, Utc::now().timestamp(), "Genesis Block".to_string(), "".to_string());
        self.chain.push(genesis_block);
    }

    pub fn add_new_block(&mut self, data: String) {
        let last_block = self.chain.last().unwrap().clone();
        let new_block = Block::new(self.current_index, Utc::now().timestamp(), data, last_block.hash().clone());
        self.chain.push(new_block);
        self.current_index += 1;
    }

    pub fn validate_chain(&self) -> bool {
        for (i, current_block) in self.chain.iter().enumerate() {
            if current_block.hash() != &current_block.calculate_hash() {
                return false;
            }
            if i != 0 {
                let prev_block = &self.chain[i - 1];
                if current_block.prev_hash() != prev_block.hash() {
                    return false;
                }
            }
        }
        true
    }

    pub fn display(&self) {
        for block in &self.chain {
            println!("Index: {}", block.index());
            println!("Timestamp: {}", block.timestamp());
            println!("Data: {}", block.data());
            println!("Prev Hash: {}", block.prev_hash());
            println!("Hash: {}", block.hash());
            println!("");
        }
    }
}
