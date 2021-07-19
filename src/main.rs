use std::convert::TryFrom;
use std::fs::{read_to_string, File};
use std::io::Write;
use std::path::PathBuf;
use structopt::StructOpt;

use mempool::error::Result;
use mempool::model::{Fee, Mempool, Transaction};

#[derive(Debug, StructOpt)]
#[structopt(name = "mempool", about = "Prioritizes transactions.")]
struct Opt {
    /// Input file of transactions
    #[structopt(parse(from_os_str))]
    input: PathBuf,

    /// Output file of prioritized transactions in mempool. stdout if not provided.
    #[structopt(short, parse(from_os_str))]
    output: Option<PathBuf>,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    let transactions = read_to_string(opt.input)?
        .split("\n")
        .map(|s| Transaction::try_from(s))
        .collect::<Result<Vec<_>>>()?;

    // We use (Fee, TxHash) as the key. This is because it is possible there can
    // be the same fee for multiple transactions, which would cause transactions
    // to be overwritten when adding entries with the same key. Thus TxHash is
    // added in order to sort by fee first, then if two entries have the same
    // fee, the smallest TxHash is removed.
    let mut pool: Mempool<(Fee, String), Transaction> = Mempool::new_with_capacity(5000);

    for transaction in transactions {
        pool.insert(
            (transaction.fee(), transaction.tx_hash.clone()),
            transaction,
        );
    }

    let mut file = if let Some(output) = opt.output {
        Some(File::create(output)?)
    } else {
        None
    };

    // Iterate through reversed to go from largest to smallest
    for transaction in pool.data.values().rev() {
        if let Some(ref mut file) = file {
            writeln!(file, "{}", transaction)?;
        } else {
            println!("{}", transaction);
        }
    }

    Ok(())
}
