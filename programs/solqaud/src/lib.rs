use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod solqaud {
    use super::*;
    pub fn start_match_pool(ctx: Context<StartMatchPool>) -> ProgramResult {
        let match_pool = &mut ctx.accounts.match_pool;
        match_pool.pool_id = 0;
        Ok(())
    }

    pub fn create_match_pool(ctx: Context<CreateMatchPool>, p_name: String) -> ProgramResult {
        let match_pool = &mut ctx.accounts.match_pool;
        let creator = &mut ctx.accounts.creator;

        let p_id = 1;

        let my_project = MyProject {
            project_id: p_id,
            project_owner: *creator.to_account_info().key,
            project_name: p_name.to_string(),
        };

        match_pool.projects_in_pool.push(my_project);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct StartMatchPool<'info> {
    #[account(mut)]
    pub match_pool: Account<'info, MatchPool>,
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateMatchPool<'info> {
    #[account(mut)]
    pub match_pool: Account<'info, MatchPool>,
    pub creator: Signer<'info>,
    // pub system_program: Program<'info, System>,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct MyProject {
    project_id: u16,
    project_owner: Pubkey,
    project_name: String,
}

#[account]
pub struct MatchPool {
    pool_id: u8,
    pool_owner: Pubkey,
    pool_amount: u64,
    projects_in_pool: Vec<MyProject>,
}

// #[derive(Accounts)]
// pub struct Initialize {

// }

// TODO: ADD voting

// TODO: ADD timelock

// TODO: ADD token transfer

#[error]
pub enum Err {
    #[msg("Pool creation error")]
    PoolCreationError
}