use coding_test::{AccountStorage, TransactionParser};
use rust_decimal::Decimal;

#[test]
fn valid_deposite_dispute_test() {
  let mut account_storage = AccountStorage::new();
  assert!(TransactionParser::parse_transactions_into_accounts(
    "transaction_files/valid_deposit_disputes.csv",
    &mut account_storage
  ));

  // disputed
  let funds_client1 = account_storage.get_mut(1).get_funds();
  assert_eq!(funds_client1.get_total(), Decimal::new(10000, 4)); // total: 1
  assert_eq!(funds_client1.available, Decimal::new(0, 4)); // available: 0
  assert_eq!(funds_client1.held, Decimal::new(10000, 4)); // held: 1
  assert!(!account_storage.get_mut(1).is_frozen());

  // resolved
  let funds_client2 = account_storage.get_mut(2).get_funds();
  assert_eq!(funds_client2.get_total(), Decimal::new(20000, 4)); // total: 2
  assert_eq!(funds_client2.available, Decimal::new(20000, 4)); // available: 2
  assert_eq!(funds_client2.held, Decimal::new(0, 4)); // held: 0
  assert!(!account_storage.get_mut(2).is_frozen());

  // charged back
  let funds_client3 = account_storage.get_mut(3).get_funds();
  assert_eq!(funds_client3.get_total(), Decimal::new(0, 4)); // total: 0
  assert_eq!(funds_client3.available, Decimal::new(0, 4)); // available: 0
  assert_eq!(funds_client3.held, Decimal::new(0, 4)); // held: 0
  assert!(account_storage.get_mut(3).is_frozen()); // this account must be frozen
}

#[test]
fn valid_withdrawal_dispute_test() {
  let mut account_storage = AccountStorage::new();
  assert!(TransactionParser::parse_transactions_into_accounts(
    "transaction_files/valid_withdrawal_disputes.csv",
    &mut account_storage
  ));

  // disputed
  let funds_client1 = account_storage.get_mut(1).get_funds();
  assert_eq!(funds_client1.get_total(), Decimal::new(10000, 4)); // total: 1
  assert_eq!(funds_client1.available, Decimal::new(0, 4)); // available: 0
  assert_eq!(funds_client1.held, Decimal::new(10000, 4)); // held: 1
  assert!(!account_storage.get_mut(1).is_frozen());

  // resolved
  let funds_client2 = account_storage.get_mut(2).get_funds();
  assert_eq!(funds_client2.get_total(), Decimal::new(20000, 4)); // total: 2
  assert_eq!(funds_client2.available, Decimal::new(20000, 4)); // available: 2
  assert_eq!(funds_client2.held, Decimal::new(0, 4)); // held: 0
  assert!(!account_storage.get_mut(2).is_frozen());

  // charged back
  let funds_client3 = account_storage.get_mut(3).get_funds();
  assert_eq!(funds_client3.get_total(), Decimal::new(0, 4)); // total: 0
  assert_eq!(funds_client3.available, Decimal::new(0, 4)); // available: 0
  assert_eq!(funds_client3.held, Decimal::new(0, 4)); // held: 0
  assert!(account_storage.get_mut(3).is_frozen()); // this account must be frozen
}

#[test]
fn invalid_dispute_test() {
  let mut account_storage = AccountStorage::new();
  assert!(TransactionParser::parse_transactions_into_accounts(
    "transaction_files/invalid_disputes.csv",
    &mut account_storage
  ));

  // resolved
  let funds_client1 = account_storage.get_mut(1).get_funds();
  assert_eq!(funds_client1.get_total(), Decimal::new(70000, 4)); // total: 7
  assert_eq!(funds_client1.available, Decimal::new(70000, 4)); // available: 7
  assert_eq!(funds_client1.held, Decimal::new(0, 4)); // held: 0
  assert!(!account_storage.get_mut(1).is_frozen());
}
