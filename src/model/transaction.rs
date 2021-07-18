use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt;

use super::Fee;
use crate::error::Error;

#[derive(Clone, Debug)]
pub struct Transaction {
    pub tx_hash: String,
    pub gas: u64,
    pub fee_per_gas: f64,
    pub signature: String,
}

impl Transaction {
    pub fn fee(&self) -> Fee {
        Fee(self.gas as f64 * self.fee_per_gas)
    }
}

impl<'a> TryFrom<&'a str> for Transaction {
    type Error = Error;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        // HashMap of key=value
        let mut kv_pairs = s
            .split(" ")
            .map(|kv| {
                let mut iter = kv.split("=");

                let key = iter
                    .next()
                    .ok_or(Error::InvalidTransactionFormat)?
                    .to_string();
                let value = iter
                    .next()
                    .ok_or(Error::InvalidTransactionFormat)?
                    .to_string();

                Ok((key, value))
            })
            .collect::<Result<HashMap<String, String>, Error>>()?;

        Ok(Transaction {
            tx_hash: kv_pairs
                .remove("TxHash")
                .ok_or(Error::InvalidTransactionFormat)?,
            gas: kv_pairs
                .remove("Gas")
                .ok_or(Error::InvalidTransactionFormat)?
                .parse()?,
            fee_per_gas: kv_pairs
                .remove("FeePerGas")
                .ok_or(Error::InvalidTransactionFormat)?
                .parse()?,
            signature: kv_pairs
                .remove("Signature")
                .ok_or(Error::InvalidTransactionFormat)?,
        })
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Fee={} TxHash={} Gas={} FeePerGas={} Signature={}",
            self.fee().0, self.tx_hash, self.gas, self.fee_per_gas, self.signature
        )
    }
}
