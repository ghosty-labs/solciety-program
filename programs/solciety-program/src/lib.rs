use anchor_lang::prelude::*;

declare_id!("5aK8HrYzjgJrut2xo48NQwbm4DcEn6cceiJjyxxhcW5E");

#[program]
pub mod solciety_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
