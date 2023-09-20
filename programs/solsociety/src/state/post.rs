use anchor_lang::prelude::*;

#[account]
#[derive(Debug)]
pub struct Post {
  pub user: Pubkey,
  pub timestamp: i64,
  pub state: Option<PostState>,
  pub tag: String,
  pub content: String,
  pub image: String
}

#[derive(Accounts)]
pub struct SendPost<'info> {
  #[account(init, payer = user, space = 8 + 32 + 8 + 1 + (4 + 50 * 4) + (4 + 280 * 4))]
  pub post: Account<'info, Post>,
  #[account(mut)]
  pub user: Signer<'info>,
  pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct UpdatePost<'info> {
  #[account(mut, has_one = user)]
  pub post: Account<'info, Post>,
  pub user: Signer<'info>
}

#[derive(Accounts)]
pub struct DeletePost<'info> {
  #[account(mut, has_one = user)]
  pub post: Account<'info, Post>,
  pub user: Signer<'info>
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Debug)]
pub enum PostState {
  Edited,
  Deleted,
}