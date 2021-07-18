use std::convert::TryFrom;
use std::error::Error as StdError;
use std::fs::{read_to_string, File};
use std::io::Write;
use std::path::PathBuf;
use structopt::StructOpt;

use mempool::error::Error;
use mempool::model::{Fee, Mempool, Transaction};

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct Opt {
    /// Activate debug mode
    // short and long flags (-d, --debug) will be deduced from the field's name
    #[structopt(short, long)]
    debug: bool,

    /// Input file
    #[structopt(parse(from_os_str))]
    input: PathBuf,

    /// Output file, stdout if not present
    #[structopt(parse(from_os_str))]
    output: PathBuf,
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

    let mut file = File::create(opt.output)?;

    // Iterate through reversed
    for transaction in pool.data.values().rev() {
        writeln!(file, "{}", transaction)?;
    }

    Ok(())
}
