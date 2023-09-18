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
}
