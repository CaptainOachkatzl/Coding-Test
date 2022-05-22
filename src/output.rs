use csv::Writer;
use log::error;
use rust_decimal::Decimal;
use serde::Serialize;

use crate::AccountStorage;

#[derive(Serialize)]
struct AccountSerializable {
  pub client: u16,
  pub available: Decimal,
  pub held: Decimal,
  pub total: Decimal,
  pub locked: bool,
}

pub fn print_output_csv(account_storage: &AccountStorage) {
  let mut writer = Writer::from_writer(std::io::stdout());

  account_storage.get_accounts().iter().for_each(|(&client_id, account)| {
    let funds = account.get_funds();
    if let Err(err) = writer.serialize(AccountSerializable {
      client: client_id,
      available: funds.available,
      held: funds.held,
      total: funds.get_total(),
      locked: account.is_frozen(),
    }) {
      error!("error while serializing account {}: {}", client_id, err);
    }
  });

  if let Err(err) = writer.flush() {
    error!("error while flushing to std::out: {}", err);
  }
}
