use coding_test::{AccountStorage, TransactionParser};

#[test]
fn large_valid_test() {
  let mut account_storage = AccountStorage::new();
  assert!(TransactionParser::parse_transactions_into_accounts(
    "transaction_files/valid_mixed.csv",
    &mut account_storage
  ));
}

#[test]
fn large_invalid_test() {
  let mut account_storage = AccountStorage::new();
  assert!(TransactionParser::parse_transactions_into_accounts(
    "transaction_files/invalid_mixed.csv",
    &mut account_storage
  ));
}
