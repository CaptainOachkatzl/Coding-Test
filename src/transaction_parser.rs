use anyhow::Result;
use csv::{Reader, ReaderBuilder};
use log::error;
use std::fs::File;

use crate::{AccountStorage, Transaction};

#[derive(Debug)]
pub struct TransactionParser {
  csv_reader: Reader<File>,
}

impl TransactionParser {
  // high level call that does all the parsing and transaction assigning to accounts in one
  pub fn parse_transactions_into_accounts(input_file_name: &str, account_storage: &mut AccountStorage) -> Result<()> {
    let mut transaction_parser = TransactionParser::initialize(&input_file_name)?;
    transaction_parser.for_each_parsable_transaction(|transaction| {
      account_storage
        .get_mut(transaction.client_id)
        .add_transaction(transaction);
    });
    Ok(())
  }

  pub fn initialize(file_name: &str) -> std::io::Result<Self> {
    let csv_reader = ReaderBuilder::new()
      .trim(csv::Trim::All)
      .flexible(true)
      .from_path(file_name)?;
    Ok(Self { csv_reader })
  }

  pub fn for_each_parsable_transaction(&mut self, action: impl FnMut(Transaction)) {
    self
      .csv_reader
      .deserialize()
      .filter_map(|deserialized_tx| {
        if let Ok(tx) = deserialized_tx {
          return Some(tx);
        } else if let Err(err) = deserialized_tx {
          error!("{}", err);
        }
        None
      })
      .for_each(action);
  }
}
