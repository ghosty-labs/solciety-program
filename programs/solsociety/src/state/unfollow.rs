use anchor_lang::prelude::*;

use super::follow::FollowState;

#[account]
#[derive(Debug)]
pub struct Unfollow {
  pub user: Pubkey,
  pub unfollowing: Pubkey,
  pub timestamp: i64,
  pub state: FollowState
}

#[derive(Accounts)]
pub struct UnfollowUser<'info> {
  #[account(init, payer = user, space = 8 + 32 + 32 + 8 + 1)]
  pub follow: Account<'info, Unfollow>,
  #[account(mut)]
  pub user: Signer<'info>,
  pub system_program: Program<'info, System>
}