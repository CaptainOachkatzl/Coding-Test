use coding_test::{AccountStorage, TransactionParser};
use rust_decimal::Decimal;

#[test]
fn account_balance_deposit_withdrawal_test() {
  let mut account_storage = AccountStorage::new();
  TransactionParser::parse_transactions_into_accounts(
    "transaction_files/transactions_example.csv",
    &mut account_storage,
  )
  .unwrap();

  let funds_client1 = account_storage.get_mut(1).get_funds();
  assert_eq!(funds_client1.get_total(), Decimal::new(15000, 4)); // total: 1.5
  assert_eq!(funds_client1.available, Decimal::new(15000, 4)); // available: 1.5
  assert_eq!(funds_client1.held, Decimal::new(0, 4)); // held: 0

  let funds_client2 = account_storage.get_mut(2).get_funds();
  assert_eq!(funds_client2.get_total(), Decimal::new(20000, 4)); // total: 2
  assert_eq!(funds_client2.available, Decimal::new(20000, 4)); // available: 2
  assert_eq!(funds_client2.held, Decimal::new(0, 4)); // held: 0
}
