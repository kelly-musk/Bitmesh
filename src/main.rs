use Bitmesh::core;
// use Bitmesh::*;
use core::*;
use mnemoni_generator::*;
use wallet::walletconnect;



fn main() {

    let seed = generate_seed_phrase();

    // wallet::walletconnect();
    // println!("wallet connected successfully{:?}", walletconnect());

    let passphrase = Some("");

    // let seed = generate_seed_and_seedphrase_by_wordcount(12,passphrase );
    println!(" {:?}", seed)
    // println!("{:?}",walletconnect())
}
