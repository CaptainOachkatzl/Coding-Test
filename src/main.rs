use std::env;

use coding_test::*;

use log::{error, info};

fn main() {
  // logging completely disabled for automated test
  // uncomment the setup line to finds the logs in the "./logs.txt" file

  // setup_logging();

  let mut account_storage = AccountStorage::new();

  if let Some(input_file) = read_filename_from_args() {
    info!("input file: {}", input_file);

    if let Err(err) = TransactionParser::parse_transactions_into_accounts(&input_file, &mut account_storage) {
      error!("parsing error: {}", err);
      return;
    }
  } else {
    error!("no input file name was provided. canceling execution.");
    return;
  }

  print_output_csv(&account_storage);
}

fn read_filename_from_args() -> Option<String> {
  // args:
  // index 0 - name of program
  // index 1 - input file name
  Some(env::args().nth(1)?)
}

#[allow(unused)]
fn setup_logging() {
  if let Err(_err) = simple_logging::log_to_file("logs.txt", log::LevelFilter::Info) {
    // continue here without printing the error to std::out to not interfere with automatic parsing at any cost
  }
}
