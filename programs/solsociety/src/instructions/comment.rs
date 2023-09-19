use anchor_lang::prelude::*;
use crate::state::comment::*;
use crate::errors::ErrorCode;

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

  Ok(())
}

pub fn update_comment(ctx: Context<UpdateComment>, new_content: String) -> Result<()> {
  let comment = &mut ctx.accounts.comment;

  require!(comment.content != new_content, ErrorCode::NothingChanged);
  require!(new_content.chars().count() <= 280, ErrorCode::TooLong); 

  comment.state = Some(CommentState::Edited);
  comment.content = new_content;

  Ok(())
}

pub fn delete_comment(ctx: Context<DeleteComment>) -> Result<()> {
  let comment = &mut ctx.accounts.comment;

  comment.state = Some(CommentState::Deleted);
  comment.content = "".to_string();

  Ok(())
}
