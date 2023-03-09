use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod solana_anchor_customized {
  use super::*;

  pub fn test_function(ctx: Context<TestFunction>) -> Result<()> {
    Ok(())
  }
}

#[derive(Accounts)]
pub struct TestFunction {}
