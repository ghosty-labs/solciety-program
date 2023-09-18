use anchor_lang::prelude::*;

#[account]
pub struct Post {
  pub user: PubKey,
  pub timestamp: i64,
  pub state: Option,
  pub tag: String,
  pub content: String,
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

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum PostState {
  Edited,
  Deleted,
}