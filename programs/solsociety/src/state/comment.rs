use anchor_lang::prelude::*;

#[account]
pub struct Comment {
  pub user: Pubkey,
  pub post: Pubkey,
  pub parent: Pubkey,
  pub timestamp: i64,
  pub state: Option<CommentState>,
  pub content: String
}

#[derive(Accounts)]
pub struct SendComment<'info> {
  #[account(init, payer = user, space = 8 + 32 + 32 + 32 + 8 + 1 + (4 + 280 * 4))]
  pub comment: Account<'info, Comment>,
  #[account(mut)]
  pub user: Signer<'info>,
  pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct UpdateComment<'info> {
    #[account(mut, has_one = user)]
    pub comment: Account<'info, Comment>,
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct DeleteComment<'info> {
    #[account(mut, has_one = user)]
    pub comment: Account<'info, Comment>,
    pub user: Signer<'info>,
}


#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum CommentState {
  Edited,
  Deleted,
}