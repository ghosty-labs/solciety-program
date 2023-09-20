use anchor_lang::prelude::*;
use crate::state::post::*;
use crate::errors::ErrorCode;
use serde_json::json;

pub fn send_post(ctx: Context<SendPost>, mut tag: String, content: String) -> Result<()> {
  let post = &mut ctx.accounts.post;
  let user: &Signer = &ctx.accounts.user;
  let timestamp: Clock = Clock::get().unwrap();

  require!(tag.chars().count() <= 50, ErrorCode::TooLong);
  require!(tag.chars().all(|c| c.is_alphanumeric() || c == '-'), ErrorCode::UnallowedChars);
  require!(content.chars().count() <= 280, ErrorCode::TooLong);
  require!(content.chars().count() > 0, ErrorCode::NoContent);

  if tag == "" {
    tag = "[untagged]".to_string()
  }

  post.user = *user.key;
  post.timestamp = timestamp.unix_timestamp;
  post.tag = tag.to_lowercase();
  post.content = content;

  let account_info = &post.to_account_info();
  let logs = json!({
    "key": account_info.key.to_string(), 
    "user": &post.user.to_string(), 
    "tag": &post.tag, 
    "content": &post.content, 
    "timestamp": &post.timestamp
  });
  msg!("LOGS_SEND_POST::{:?}", serde_json::to_string_pretty(&logs));

  Ok(())
}

pub fn update_post(ctx: Context<UpdatePost>, new_tag: String, new_content: String) -> Result<()> {
  let post = &mut ctx.accounts.post;

  require!(new_tag != post.tag || new_content != post.content, ErrorCode::NothingChanged);
  require!(new_tag.chars().count() <= 50, ErrorCode::TooLong);
  require!(new_tag.chars().all(|c| c.is_alphanumeric() || c == '-'), ErrorCode::UnallowedChars);
  require!(new_content.chars().count() <= 280, ErrorCode::TooLong);
  require!(new_content.chars().count() > 0, ErrorCode::NoContent);

  post.state = Some(PostState::Edited);
  post.tag = new_tag;
  post.content = new_content;

  let account_info = &post.to_account_info();
  let logs = json!({
    "key": account_info.key.to_string(), 
    "tag": &post.tag, 
    "content": &post.content, 
  });
  msg!("LOGS_UPDATE_POST::{:?}", serde_json::to_string_pretty(&logs)); 

  Ok(())
}

pub fn delete_post(ctx: Context<DeletePost>) -> Result<()> {
  let post = &mut ctx.accounts.post;

  post.state = Some(PostState::Deleted);
  post.tag = "[deleted]".to_string();
  post.content = "".to_string();

  let account_info = &post.to_account_info();
  let logs = json!({
    "key": account_info.key.to_string(), 
    "tag": &post.tag, 
    "content": &post.content, 
  });
  msg!("LOGS_DELETE_POST::{:?}", serde_json::to_string_pretty(&logs)); 

  Ok(())
}