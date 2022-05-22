use std::collections::HashMap;

use anyhow::{bail, Result};
use log::error;
use rust_decimal::Decimal;

use crate::{BalanceTransaction, Chargeback, Deposit, Dispute, Resolve, Transaction, TransactionType, Withdrawal};

const TRANSACTION_NOT_FOUND: &str = "partner error - referenced transaction not found";

#[derive(Clone, Copy)]
pub struct Funds {
  pub available: Decimal,
  pub held: Decimal,
}

impl Funds {
  pub fn zero() -> Self {
    Self {
      available: Decimal::new(0, 4),
      held: Decimal::new(0, 4),
    }
  }

  pub fn get_total(&self) -> Decimal {
    self.available + self.held
  }
}

pub struct Account {
  funds: Funds,
  balance_transactions: HashMap<u32, Box<dyn BalanceTransaction>>,
  frozen: bool,
}

impl Account {
  pub fn new() -> Self {
    Self {
      funds: Funds::zero(),
      frozen: false,
      balance_transactions: HashMap::new(),
    }
  }

  pub fn get_funds(&self) -> Funds {
    self.funds
  }

  pub fn is_frozen(&self) -> bool {
    self.frozen
  }

  pub fn add_transaction(&mut self, transaction: Transaction) {
    if self.is_frozen() {
      error!(
        "unable to handle transaction {} for account {}: account is frozen",
        transaction.tx_id,
        transaction.client_id,
      );
      return;
    }

    if let Err(err) = self.add_transaction_as_converted(transaction) {
      error!("unable to handle transaction {} for account {}: {}", transaction.tx_id, transaction.client_id, err);
    }
  }

  fn add_transaction_as_converted(&mut self, transaction: Transaction) -> Result<()> {
    match transaction.tx_type {
      TransactionType::Deposit => {
        self.handle_deposit(transaction.try_into()?);
      }
      TransactionType::Withdrawal => {
        self.handle_withdrawal(transaction.try_into()?)?;
      }
      TransactionType::Dispute => {
        self.handle_dispute(&transaction.try_into()?)?;
      }
      TransactionType::Resolve => {
        self.handle_resolve(&transaction.try_into()?)?;
      }
      TransactionType::Chargeback => {
        self.handle_chargeback(&transaction.try_into()?)?;
      }
    };
    Ok(())
  }

  fn handle_deposit(&mut self, deposit: Deposit) {
    self.funds.available += deposit.get_amount();
    self.balance_transactions.insert(deposit.get_tx_id(), Box::new(deposit));
  }

  fn handle_withdrawal(&mut self, withdrawal: Withdrawal) -> Result<()> {
    let amount = withdrawal.get_amount();

    if (self.funds.available - amount).is_sign_negative() {
      bail!("not enough funds available to withdraw");
    }

    self.funds.available -= amount;
    self
      .balance_transactions
      .insert(withdrawal.get_tx_id(), Box::new(withdrawal));
    Ok(())
  }

  fn handle_dispute(&mut self, dispute: &Dispute) -> Result<()> {
    if let Some(balance_transaction) = self.balance_transactions.get_mut(&dispute.get_tx_id()) {
      balance_transaction.dispute(&mut self.funds)?;
      Ok(())
    } else {
      bail!(TRANSACTION_NOT_FOUND);
    }
  }

  fn handle_resolve(&mut self, resolve: &Resolve) -> Result<()> {
    if let Some(balance_transaction) = self.balance_transactions.get_mut(&resolve.get_tx_id()) {
      balance_transaction.resolve(&mut self.funds)?;
      Ok(())
    } else {
      bail!(TRANSACTION_NOT_FOUND);
    }
  }

  fn handle_chargeback(&mut self, chargeback: &Chargeback) -> Result<()> {
    if let Some(balance_transaction) = self.balance_transactions.get_mut(&chargeback.get_tx_id()) {
      balance_transaction.chargeback(&mut self.funds)?;
      self.frozen = true;
      Ok(())
    } else {
      bail!(TRANSACTION_NOT_FOUND);
    }
  }
}
