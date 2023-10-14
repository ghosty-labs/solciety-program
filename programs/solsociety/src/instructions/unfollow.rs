use anchor_lang::prelude::*;
use crate::state::{follow::*, unfollow::UnfollowUser};
use serde_json::json;

pub fn unfollow_user(ctx: Context<UnfollowUser>, unfollowing: Pubkey) -> Result<()> {
  let follow = &mut ctx.accounts.follow;
  let user: &Signer = &ctx.accounts.user;
  let timestamp: Clock = Clock::get().unwrap();

  follow.user = *user.key;
  follow.unfollowing = unfollowing;
  follow.state = FollowState::Unfollowed;
  follow.timestamp = timestamp.unix_timestamp;

  let account_info = &follow.to_account_info();
  let logs = json!({
    "key": account_info.key.to_string(),
    "user": &follow.user.to_string(),
    "unfollowing": &follow.unfollowing.to_string(),
    "state": &follow.state,
    "timestamp": &follow.timestamp
  });
  msg!("SOLCIETYLOGS_UNFOLLOWUSER::{:?}", serde_json::to_string_pretty(&logs));

  Ok(())
}