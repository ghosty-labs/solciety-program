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
}
