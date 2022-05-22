use anyhow::{bail, Result};

pub struct DisputeStatus {
  disputed: bool,
  charged_back: bool,
}

impl DisputeStatus {
  pub fn new() -> Self {
    Self {
      disputed: false,
      charged_back: false,
    }
  }

  pub fn dispute(&mut self) -> Result<()> {
    if self.disputed {
      bail!("already disputed");
    }

    self.disputed = true;
    Ok(())
  }

  pub fn resolve(&mut self) -> Result<()> {
    self.verify_disputed()?;

    self.disputed = false;
    Ok(())
  }

  pub fn chargeback(&mut self) -> Result<()> {
    self.verify_disputed()?;

    self.charged_back = true;
    Ok(())
  }

  fn verify_disputed(&self) -> Result<()> {
    if !self.disputed {
      bail!("not disputed");
    }

    if self.charged_back {
      bail!("already charged back")
    }

    Ok(())
  }
}
