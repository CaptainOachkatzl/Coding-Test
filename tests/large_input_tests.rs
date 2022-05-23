use coding_test::{Account, AccountStorage, TransactionParser};

#[test]
fn large_valid_test() {
  let mut account_storage = AccountStorage::new();
  TransactionParser::parse_transactions_into_accounts("transaction_files/valid_mixed.csv", &mut account_storage).unwrap();
  account_storage
    .get_accounts()
    .iter()
    .for_each(|(_, acc)| assert_positive_balance(acc));
}

#[test]
fn large_invalid_test() {
  let mut account_storage = AccountStorage::new();
  TransactionParser::parse_transactions_into_accounts("transaction_files/invalid_mixed.csv", &mut account_storage).unwrap();
  account_storage
    .get_accounts()
    .iter()
    .for_each(|(_, acc)| assert_positive_balance(acc));
}

fn assert_positive_balance(account: &Account) {
  assert!(account.get_funds().available.is_sign_positive());
  assert!(account.get_funds().held.is_sign_positive());
  assert!(account.get_funds().get_total().is_sign_positive());
}
