use std::path::PathBuf;
use structopt::StructOpt;

extern crate hex;
extern crate parity_crypto;
extern crate secp256k1;
extern crate web3;

mod key;
mod transfer;
mod receive;
mod errors;
mod utils;

/// RsTx client for Stealth Addresses in Ethereum
#[derive(StructOpt, Debug)]
#[structopt(name = "RsTx Client")]
enum Cli {
    /// Create a new keypair
    #[structopt(name = "create")]
    Create {
        /// Directory to store
        /// the created keypair
        #[structopt(short = "s", parse(from_os_str))]
        storage_dir: PathBuf
    },
    /// List all ECDH keypairs
    #[structopt(name = "list")]
    List {
        /// Directory from which
        /// to list keypairs
        #[structopt(short = "s", parse(from_os_str))]
        storage_dir: PathBuf
    },
    /// Transfer ether
    #[structopt(name = "transfer")]
    Transfer {
        /// Directory in which
        /// keypair file is saved
        #[structopt(short = "s", parse(from_os_str))]
        storage_dir: PathBuf,
        /// Sender address, will be loaded
        /// from the key in storage_dir
        #[structopt(short = "f")]
        from: String,
        /// Recipient public key
        /// in compressed form
        #[structopt(short = "t")]
        to: String,
        /// Value to be transferred
        /// (in wei)
        #[structopt(short = "v")]
        value: String
    },
    /// Receive ether
    #[structopt(name = "receive")]
    Receive {
        /// Directory in which master
        /// keypair file is saved
        #[structopt(short = "s", parse(from_os_str))]
        storage_dir: PathBuf,
        /// Master key address
        #[structopt(short = "a")]
        address: String,
        /// Nonce point (in compressed form)
        /// of the stealth transaction
        #[structopt(short = "n")]
        nonce_point: String
    },
    /// Scan the blockchain
    /// for incoming txs
    #[structopt(name = "scan")]
    Scan {
        /// Block number to
        /// scan from
        #[structopt(short = "b")]
        block: Option<i32>
    }
}

fn main() {
    match Cli::from_args() {
        Cli::Create { mut storage_dir } => {
            println!("Handle Create {:?}", storage_dir);
            if let Err(error) = key::new(&mut storage_dir) {
                panic!("[Error in creating/storing keypair]: {:?}", error)
            }
        },
        Cli::List { storage_dir } => println!("Handle List {:?}", storage_dir),
        Cli::Transfer { mut storage_dir, from, to, value } => {
            println!("Handle Transfer [dir] = {:?}, [from] = {}, [to] = {}, value = {}", storage_dir, from, to, value);
            match transfer::transfer(&mut storage_dir, &from, &to, &value) {
                Ok(transferReceipt) => {
                    println!("Successfully transferred");
                    println!("Tx hash: {:?}", transferReceipt.tx_hash);
                    println!("Share this nonce point with recipient: {:x}", transferReceipt.nonce_point_compressed);
                    println!("nonce point (x) = {:?}", transferReceipt.nonce_point_x);
                    println!("nonce point (y) = {:?}", transferReceipt.nonce_point_y);
                },
                Err(error) => panic!("[Error in transfer]: {:?}", error)
            }
        },
        Cli::Receive { mut storage_dir, address, nonce_point } => {
            println!("Handle receive [dir] = {:?}, [master] = {}, [nonce point] = {}", storage_dir, address, nonce_point);
            match receive::receive(&mut storage_dir, &address, &nonce_point) {
                Ok(receipt) => {
                    println!("Successfully claimed receipt");
                    println!("Recipient address: {:?}", receipt.address);
                    println!("Recipient balance: {:?}", receipt.balance);
                },
                Err(error) => panic!("[Error in receiving]: {:?}", error)
            }
        },
        Cli::Scan { block } => {
            if let Some(b) = block {
                println!("Handle Scan from block {:?}", b);
            } else {
                println!("Handle Scan from last block");
            }
        }
    }
}
