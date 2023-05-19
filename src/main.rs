mod block;
mod blockchain;

use blockchain::Blockchain;

fn main() {
    let mut blockchain = Blockchain::new();
    blockchain.add_new_block("Some data for the first block".to_string());
    blockchain.add_new_block("Some data for the second block".to_string());

    println!("{:#?}", blockchain);
}
