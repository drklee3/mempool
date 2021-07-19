# mempool

A priority [mempool](https://www.blocknative.com/blog/mempool-intro) implemented
in Rust.

## Building

```bash
cargo build --release
```

## Usage

```text
    mempool [OPTIONS] <input>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -o <output>        Output file of prioritized transactions in mempool. stdout if not provided

ARGS:
    <input>    Input file of transactions
```

## Examples

```bash
# Print prioritized transactions to stdout
./target/release/mempool transactions.txt

# Write transactions to file
./target/release/mempool transactions.txt -o prioritized-transactions.txt
```

## Benchmarking

There are benchmarks for running a mempool of size 5000. One sequentially inserts
items from 0..10000 and second one is 10000 random numbers.

Benchmarks can be run with the following command.

```bash
cargo bench
```
