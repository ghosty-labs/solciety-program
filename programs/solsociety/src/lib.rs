use anchor_lang::prelude::*;
use state::*;

pub mod state;
pub mod errors;
pub mod instructions;

declare_id!("6sTexXR4daCeaGPL6dBpaVhadBMjU9fMkpUhSP4MGEEs");

#[program] 
pub mod solsociety {
  use super::*;

  pub fn send_post(ctx: Context<SendPost>, tag: String, content: String) -> Result<()> {
    instructions::send_post(ctx, tag, content)
  }

  pub fn update_post(ctx: Context<UpdatePost>, new_tag: String, new_content: String) -> Result<()> {
    instructions::update_post(ctx, new_tag, new_content)
  }

  pub fn delete_post(ctx: Context<DeletePost>) -> Result<()> {
    instructions::delete_post(ctx)
  }

  pub fn send_comment(ctx: Context<SendComment>, post: Pubkey, content: String, parent: Option<Pubkey>) -> Result<()> {
    instructions::send_comment(ctx, post, content, parent)
  }

  pub fn update_comment(ctx: Context<UpdateComment>, new_content: String) -> Result<()> {
    instructions::update_comment(ctx, new_content)
  }

  pub fn delete_comment(ctx: Context<DeleteComment>) -> Result<()> {
    instructions::delete_comment(ctx)
  }

  pub fn follow_user(ctx: Context<FollowUser>, following: Pubkey) -> Result<()> {
    instructions::follow_user(ctx, following)
  }

  pub fn unfollow_user(ctx: Context<UnfollowUser>, unfollowing: Pubkey) -> Result<()> {
    instructions::unfollow_user(ctx, unfollowing)
  }
}
