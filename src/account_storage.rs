use std::collections::HashMap;

use crate::account::Account;

pub struct AccountStorage {
  accounts: HashMap<u16, Account>,
}

impl AccountStorage {
  pub fn new() -> Self {
    Self {
      accounts: HashMap::new(),
    }
  }

  pub fn get_mut(&mut self, client_id: u16) -> &mut Account {
    self.accounts.entry(client_id).or_insert(Account::new())
  }

  pub fn registered_client_count(&self) -> usize {
    self.accounts.len()
  }

  pub fn get_accounts(&self) -> &HashMap<u16, Account> {
    &self.accounts
  }
}
