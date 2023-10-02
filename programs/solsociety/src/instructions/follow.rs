use anchor_lang::prelude::*;
use crate::state::follow::*;
use serde_json::json;

pub fn follow_user(ctx: Context<FollowUser>, following: Pubkey) -> Result<()> {
  let follow = &mut ctx.accounts.follow;
  let user: &Signer = &ctx.accounts.user;
  let timestamp: Clock = Clock::get().unwrap();

  follow.user = *user.key;
  follow.following = following;
  follow.state = FollowState::Followed;
  follow.timestamp = timestamp.unix_timestamp;

  let account_info = &follow.to_account_info();
  let logs = json!({
    "key": account_info.key.to_string(),
    "user": &follow.user.to_string(),
    "following": &follow.following.to_string(),
    "state": &follow.state,
    "timestamp": &follow.timestamp
  });
  msg!("SOLCIETYLOGS_FOLLOWUSER::{:?}", serde_json::to_string_pretty(&logs));

  Ok(())
}