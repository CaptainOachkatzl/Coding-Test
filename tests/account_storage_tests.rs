use coding_test::AccountStorage;
use rust_decimal::Decimal;

#[test]
fn account_creation_test() {
  let valid_client_ids = [0, 5, 0, 5, 12575];

  let mut storage = AccountStorage::new();

  valid_client_ids.iter().for_each(|&client_id| {
    let account = storage.get_mut(client_id);
    assert_eq!(account.get_funds().get_total(), Decimal::new(0, 4));
  });

  assert_eq!(storage.registered_client_count(), 3);
}
