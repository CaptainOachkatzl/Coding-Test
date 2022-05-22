use coding_test::{parse_transactions_into_accounts, print_output_csv, AccountStorage, TransactionParser};

#[test]
fn transaction_reader_test() {
  let mut counter = 0;
  let mut transaction_parser = TransactionParser::initialize("transaction_files/transactions_example.csv").unwrap();
  transaction_parser.for_each_parsable_transaction(|_| {
    counter += 1;
  });

  assert_eq!(counter, 5);
}

#[test]
fn csv_output_test() {
  let mut account_storage = AccountStorage::new();
  parse_transactions_into_accounts("transaction_files/transactions_example.csv", &mut account_storage);
  print_output_csv(&account_storage);
}
