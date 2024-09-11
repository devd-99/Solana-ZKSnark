#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("4wxoA7h51hNkAgnVgXx2m3u7fx2igDbB6FMNWTyXVAoi");

#[program]
pub mod simple_zk {
    use super::*;

  pub fn close(_ctx: Context<CloseSimpleZk>) -> Result<()> {
    Ok(())
  }

  pub fn decrement(ctx: Context<Update>) -> Result<()> {
    ctx.accounts.simple_zk.count = ctx.accounts.simple_zk.count.checked_sub(1).unwrap();
    Ok(())
  }

  pub fn increment(ctx: Context<Update>) -> Result<()> {
    ctx.accounts.simple_zk.count = ctx.accounts.simple_zk.count.checked_add(1).unwrap();
    Ok(())
  }

  pub fn initialize(_ctx: Context<InitializeSimpleZk>) -> Result<()> {
    Ok(())
  }

  pub fn set(ctx: Context<Update>, value: u8) -> Result<()> {
    ctx.accounts.simple_zk.count = value.clone();
    Ok(())
  }
}

#[derive(Accounts)]
pub struct InitializeSimpleZk<'info> {
  #[account(mut)]
  pub payer: Signer<'info>,

  #[account(
  init,
  space = 8 + SimpleZk::INIT_SPACE,
  payer = payer
  )]
  pub simple_zk: Account<'info, SimpleZk>,
  pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct CloseSimpleZk<'info> {
  #[account(mut)]
  pub payer: Signer<'info>,

  #[account(
  mut,
  close = payer, // close account and return lamports to payer
  )]
  pub simple_zk: Account<'info, SimpleZk>,
}

#[derive(Accounts)]
pub struct Update<'info> {
  #[account(mut)]
  pub simple_zk: Account<'info, SimpleZk>,
}

#[account]
#[derive(InitSpace)]
pub struct SimpleZk {
  count: u8,
}
