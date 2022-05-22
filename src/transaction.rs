use anyhow::{bail, Result};
use rust_decimal::Decimal;
use serde::Deserialize;

use crate::{DisputeStatus, Funds};

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum TransactionType {
  Deposit,
  Withdrawal,
  Dispute,
  Resolve,
  Chargeback,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Copy)]
pub struct Transaction {
  #[serde(rename = "type")]
  pub tx_type: TransactionType,
  #[serde(rename = "client")]
  pub client_id: u16,
  #[serde(rename = "tx")]
  pub tx_id: u32,
  pub amount: Option<Decimal>,
}

pub trait BalanceTransaction {
  fn get_tx_id(&self) -> u32;
  fn get_amount(&self) -> Decimal;
  fn dispute(&mut self, funds: &mut Funds) -> Result<()>;
  fn resolve(&mut self, funds: &mut Funds) -> Result<()>;
  fn chargeback(&mut self, funds: &mut Funds) -> Result<()>;
}

pub struct Deposit {
  tx_id: u32,
  amount: Decimal,
  dispute_status: DisputeStatus,
}

impl BalanceTransaction for Deposit {
  fn get_tx_id(&self) -> u32 {
    self.tx_id
  }
  fn get_amount(&self) -> Decimal {
    self.amount
  }

  fn dispute(&mut self, funds: &mut Funds) -> Result<()> {
    self.dispute_status.dispute()?;
    funds.held += self.get_amount();
    funds.available -= self.get_amount();
    Ok(())
  }

  fn resolve(&mut self, funds: &mut Funds) -> Result<()> {
    self.dispute_status.resolve()?;
    funds.available += self.get_amount();
    funds.held -= self.get_amount();
    Ok(())
  }

  fn chargeback(&mut self, funds: &mut Funds) -> Result<()> {
    self.dispute_status.chargeback()?;
    funds.held -= self.get_amount();
    Ok(())
  }
}

impl TryInto<Deposit> for Transaction {
  type Error = anyhow::Error;

  fn try_into(self) -> Result<Deposit> {
    if self.tx_type != TransactionType::Deposit {
      bail!("invalid transaction type");
    }

    if let Some(amount) = self.amount {
      if amount.is_sign_negative() {
        bail!("negative value for amount");
      }

      return Ok(Deposit {
        tx_id: self.tx_id,
        amount,
        dispute_status: DisputeStatus::new(),
      });
    }
    bail!("amount must not be None for deposits")
  }
}

pub struct Withdrawal {
  tx_id: u32,
  amount: Decimal,
  dispute_status: DisputeStatus,
}

impl BalanceTransaction for Withdrawal {
  fn get_tx_id(&self) -> u32 {
    self.tx_id
  }
  fn get_amount(&self) -> Decimal {
    self.amount
  }

  fn dispute(&mut self, funds: &mut Funds) -> Result<()> {
    self.dispute_status.dispute()?;
    funds.held += self.get_amount();
    Ok(())
  }

  fn resolve(&mut self, funds: &mut Funds) -> Result<()> {
    self.dispute_status.resolve()?;
    funds.available += self.get_amount();
    funds.held -= self.get_amount();
    Ok(())
  }

  fn chargeback(&mut self, funds: &mut Funds) -> Result<()> {
    self.dispute_status.chargeback()?;
    funds.held -= self.get_amount();
    Ok(())
  }
}

impl TryInto<Withdrawal> for Transaction {
  type Error = anyhow::Error;

  fn try_into(self) -> Result<Withdrawal> {
    if self.tx_type != TransactionType::Withdrawal {
      bail!("invalid transaction type");
    }

    if let Some(amount) = self.amount {
      if amount.is_sign_negative() {
        bail!("negative value for amount");
      }

      return Ok(Withdrawal {
        tx_id: self.tx_id,
        amount,
        dispute_status: DisputeStatus::new(),
      });
    }
    bail!("amount must not be None for withdrawals")
  }
}

pub struct Dispute {
  tx_id: u32,
}

impl Dispute {
  pub fn get_tx_id(&self) -> u32 {
    self.tx_id
  }
}

impl TryInto<Dispute> for Transaction {
  type Error = anyhow::Error;

  fn try_into(self) -> Result<Dispute> {
    if self.tx_type != TransactionType::Dispute {
      bail!("invalid transaction type");
    }
    Ok(Dispute { tx_id: self.tx_id })
  }
}

pub struct Resolve {
  tx_id: u32,
}

impl Resolve {
  pub fn get_tx_id(&self) -> u32 {
    self.tx_id
  }
}

impl TryInto<Resolve> for Transaction {
  type Error = anyhow::Error;

  fn try_into(self) -> Result<Resolve> {
    if self.tx_type != TransactionType::Resolve {
      bail!("invalid transaction type");
    }
    Ok(Resolve { tx_id: self.tx_id })
  }
}

pub struct Chargeback {
  tx_id: u32,
}

impl Chargeback {
  pub fn get_tx_id(&self) -> u32 {
    self.tx_id
  }
}

impl TryInto<Chargeback> for Transaction {
  type Error = anyhow::Error;

  fn try_into(self) -> Result<Chargeback> {
    if self.tx_type != TransactionType::Chargeback {
      bail!("invalid transaction type");
    }
    Ok(Chargeback { tx_id: self.tx_id })
  }
}
