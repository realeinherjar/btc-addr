use bdk::bitcoin::util::bip32::{ExtendedPubKey, Fingerprint};
use bdk::bitcoin::Network;
use bdk::database::MemoryDatabase;
use bdk::template::Bip84Public;
use bdk::wallet::AddressIndex;
use bdk::{KeychainKind, Wallet};
use clap::Parser;
use std::str::FromStr;

/// Bitcoin address generator
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Extended public key
    #[arg(short, long)]
    xpub: String,

    /// Fingerprint of the master key
    #[arg(short, long)]
    fingerprint: String,

    /// Index of the address to generate
    #[arg(short, long, group = "index")]
    single: Option<u32>,

    /// Range of addresses to generate (start-end)
    #[arg(short, long, group = "index")]
    range: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // create the wallet from the xpub
    let xpub = ExtendedPubKey::from_str(&args.xpub)?;
    let fingerprint = Fingerprint::from_str(&args.fingerprint)?;
    let wallet = Wallet::new(
        Bip84Public(xpub, fingerprint, KeychainKind::External),
        Some(Bip84Public(xpub, fingerprint, KeychainKind::External)),
        Network::Bitcoin,
        MemoryDatabase::default(),
    )?;

    // generate the address either from a single index or a range
    if let Some(index) = args.single {
        let address = wallet.get_address(AddressIndex::Peek(index))?;
        println!("{}", address);
    } else if let Some(range) = args.range {
        let parts: Vec<&str> = range.split('-').collect();
        if parts.len() != 2 {
            return Err("Invalid range format. Should be start-end.".into());
        }
        let start = parts[0].parse::<u32>()?;
        let end = parts[1].parse::<u32>()?;
        for i in start..=end {
            let address = wallet.get_address(AddressIndex::Peek(i))?;
            println!("{}", address);
        }
    }

    Ok(())
}
