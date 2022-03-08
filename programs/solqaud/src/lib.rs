use anchor_lang::prelude::*;
use anchor_spl::token::{self, CloseAccount, Mint, SetAuthority, TokenAccount, Transfer};
use spl_token::instruction::AuthorityType;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod solqaud {
    use super::*;

    pub fn create_match_pool(ctx: Context<CreateMatchPool>, amount: u64, start: i64, end: i64) -> ProgramResult {
        let match_pool = &mut ctx.accounts.match_pool;
        // let creator = &mut ctx.accounts.creator;
        let mut pool_id = 1;

        // match_pool.creator = &mut ctx.accounts.creator;
        match_pool.pool_id = pool_id;
        match_pool.pool_amount = amount;
        match_pool.start_time = start;
        match_pool.end_time = end;

        pool_id += 1;
 
        Ok(())
    }

    pub fn add_projects_in_pool(ctx: Context<AddProjectsInPool>, p_name: String) -> ProgramResult {
        let match_pool = &mut ctx.accounts.match_pool;
        let project_owner = &mut ctx.accounts.project_owner;
        let mut project_id = 1;
        
        let my_project = MyProject {
            project_id: project_id,
            project_key: *project_owner.to_account_info().key,
            project_name: p_name.to_string(),
            votes: 0,
            amount_by_voters: 0,
        };

        match_pool.projects_in_pool.push(my_project);
        project_id += 1;
        
        Ok(())
    }

    pub fn voting_started(ctx: Context<VotingStarted>, project_id: u8, amount: u64) -> ProgramResult {
        let match_pool = &mut ctx.accounts.match_pool;
        let voter = &mut ctx.accounts.voter;

        voter.voter = *voter.to_account_info().key;
        let project_id = project_id as u8;

        if i64::from(match_pool.start_time) > Clock::get().unwrap().unix_timestamp && i64::from(match_pool.end_time) < Clock::get().unwrap().unix_timestamp {
            let project = &match_pool.projects_in_pool.iter_mut().find(|x| x.project_id == project_id);
            match project {
                Some(ref mut project) => {
                    project.votes += 1;
                    project.amount_by_voters += amount; 
                },
                None => {
                    return Err(Errors::ProjectNotFoundError);
                }
            }

            voter.project_voted_to.push(project.project_key);


            anchor_lang::solana_program::program::invoke(
                &anchor_lang::solana_program::system_instruction::transfer(
                    voter.to_account_info().key,
                    project.to_account_info().key,
                    amount,
                ),
                &[
                    voter.to_account_info(),
                    project.to_account_info(),
                    ctx.accounts.system_program.to_account_info()
                ],
            )?;
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateMatchPool<'info> {
    #[account(mut)]
    pub match_pool: Account<'info, MatchPool>,
    pub creator: AccountInfo<'info>,
    pub pool_account: Account<'info, TokenAccount>,
    pub initializer_deposit_token_account: Account<'info, TokenAccount>,
    pub token_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreateMatchPool<'info> {
    fn transfer_to_pool(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.initializer_deposit_token_account
                    .to_account_info()
                    .clone(),
            to: self.pool_account
                    .to_account_info()
                    .clone(),
            authority: self.creator.clone(),
        };
        CpiContext::new(self.token_program.clone(), cpi_accounts)
    }

    fn set_authority_context(&self) -> CpiContext<'_, '_, '_, 'info, SetAuthority<'info>> {
        let cpi_accounts = SetAuthority {
            account_or_mint: self.pool_account.to_account_info().clone(),
            current_authority: self.creator.clone(),
        };
        CpiContext::new(self.token_program.clone(), cpi_accounts)
    }
}

#[derive(Accounts)]
pub struct VotingStarted<'info> {
    #[account(mut)]
    pub voter: Account<'info, Votes>,
    pub match_pool: Account<'info, MatchPool>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)] 
pub struct AddProjectsInPool<'info> {
    #[account(mut)]
    pub match_pool: Account<'info, MatchPool>,
    pub project_owner: Account<'info, MyProject>,
}

// #[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
#[account]
pub struct MyProject {
    pub project_id: u8,
    pub project_key: Pubkey,
    pub project_name: String,
    pub votes: u64,
    pub amount_by_voters: u64,
}

#[account]
pub struct MatchPool {
    pool_id: u8,
    pool_owner: Pubkey,
    pool_amount: u64,
    start_time: i64,
    end_time: i64,
    projects_in_pool: Vec<MyProject>,
}

#[account]
pub struct Votes {
    pub voter: Pubkey,
    pub project_voted_to: Vec<Pubkey>,
}

#[error]
pub enum Errors {
    #[msg("Pool creation error")]
    PoolCreationError,
    #[msg("Project not found error")]
    ProjectNotFoundError,
}