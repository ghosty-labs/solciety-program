use anchor_lang::prelude::*;
use crate::state::comment::*;
use crate::errors::ErrorCode;
use serde_json::json;

pub fn send_comment(ctx: Context<SendComment>, post: Pubkey, content: String, parent: Option<Pubkey>) -> Result<()> {
  let comment = &mut ctx.accounts.comment;
  let user: &Signer = &ctx.accounts.user;
  let clock: Clock = Clock::get().unwrap();

  require!(content.chars().count() <= 280, ErrorCode::TooLong);

  comment.user = *user.key;
  comment.post = post;
  comment.parent = parent.unwrap_or(post);
  comment.timestamp = clock.unix_timestamp;
  comment.content = content;

  let account_info = &comment.to_account_info();
  let logs = json!({
    "key": account_info.key.to_string(), 
    "user": &comment.user.to_string(),
    "post": &comment.post.to_string(),
    "parent": &comment.parent.to_string(),
    "content": &comment.content, 
  });
  msg!("SOLCIETYLOGS_SENDCOMMENT::{:?}", serde_json::to_string_pretty(&logs)); 

  Ok(())
}

pub fn update_comment(ctx: Context<UpdateComment>, new_content: String) -> Result<()> {
  let comment = &mut ctx.accounts.comment;

  require!(comment.content != new_content, ErrorCode::NothingChanged);
  require!(new_content.chars().count() <= 280, ErrorCode::TooLong); 

  comment.state = Some(CommentState::Edited);
  comment.content = new_content;

  let account_info = &comment.to_account_info();
  let logs = json!({
    "key": account_info.key.to_string(), 
    "content": &comment.content, 
  });
  msg!("SOLCIETYLOGS_UPDATECOMMENT::{:?}", serde_json::to_string_pretty(&logs)); 

  Ok(())
}

pub fn delete_comment(ctx: Context<DeleteComment>) -> Result<()> {
  let comment = &mut ctx.accounts.comment;

  comment.state = Some(CommentState::Deleted);
  comment.content = "".to_string();

  let account_info = &comment.to_account_info();
  let logs = json!({
    "key": account_info.key.to_string(), 
    "content": &comment.content, 
  });
  msg!("SOLCIETYLOGS_DELETECOMMENT::{:?}", serde_json::to_string_pretty(&logs)); 

  Ok(())
}
