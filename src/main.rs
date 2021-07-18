use std::convert::TryFrom;
use std::error::Error as StdError;
use std::fs::{read_to_string, File};
use std::io::Write;
use std::path::PathBuf;
use structopt::StructOpt;

use mempool::error::Error;
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

fn main() -> Result<(), Box<dyn StdError>> {
    let opt = Opt::from_args();
    println!("{:?}", &opt);

    let transactions = read_to_string(opt.input)?
        .split("\n")
        .map(|s| Transaction::try_from(s))
        .collect::<Result<Vec<_>, Error>>()?;

    let mut pool: Mempool<Fee, Transaction> = Mempool::new_with_capacity(5000);

    for transaction in transactions {
        pool.insert(transaction.fee(), transaction);
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
