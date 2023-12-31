use anchor_lang::prelude::*;
use serde::Serialize;

#[account]
#[derive(Debug)]
pub struct Follow {
  pub user: Pubkey,
  pub following: Pubkey,
  pub timestamp: i64,
  pub state: FollowState
}

#[derive(Accounts)]
pub struct FollowUser<'info> {
  #[account(init, payer = user, space = 8 + 32 + 32 + 8 + 1)]
  pub follow: Account<'info, Follow>,
  #[account(mut)]
  pub user: Signer<'info>,
  pub system_program: Program<'info, System>
}

#[derive(AnchorSerialize, AnchorDeserialize, Serialize, Clone, PartialEq, Eq, Debug)]
pub enum FollowState {
  Followed,
  Unfollowed,
}