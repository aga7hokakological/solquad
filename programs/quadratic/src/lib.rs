use anchor_lang::prelude::*;
use anchor_spl::token::{self, CloseAccount, Mint, SetAuthority, TokenAccount, Transfer};
use spl_token::instruction::AuthorityType;
use std::ptr::null;
use anchor_lang::__private::Error;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod quadratic {
    use super::*;

    const POOL_PDA_SEED: &[u8] = b"pool";

    pub fn create_match_pool(
        ctx: Context<CreateMatchPool>, 
        _vault_account_bump: u8,
        initializer_amount: u64,
    ) -> ProgramResult {
        ctx.accounts.pool_account.pool_creator = *ctx.accounts.pool_creator.key;

        ctx.accounts.pool_account.pool_amount_token = *ctx
            .accounts.pool_amount_token.to_account_info().key;

        ctx.accounts.pool_account.pool_amount = initializer_amount;

        let (pool_authority, _pool_authority_bump) = 
            Pubkey::find_program_address(&[POOL_PDA_SEED], ctx.program_id);
            anchor_spl::token::set_authority(
                ctx.accounts.into_set_authority_context(),
                AuthorityType::AccountOwner,
                Some(pool_authority),
            )?;

            anchor_spl::token::transfer(
                ctx.accounts.into_transfer_to_pda_context(),
                ctx.accounts.pool_account.pool_amount,
            )?;

        Ok(())
    }

    pub fn create_project(ctx: Context<CreateProject>) -> Result<(), Error> {
        let mut idx = 1;
        let project = &mut ctx.accounts.project.load_mut()?;
        project.my_project[idx as usize] = Project {
            project_id: idx,
            // project_owner: *ctx.accounts.project_owner.key,
            // project_amount_by_voters_token: *ctx
            //     .accounts.project_amount_by_voters_token
            //     .to_account_info().key,
            project_amount_by_voters: 0,
            total_votes: 0,
        };

        idx += 1;

        Ok(())
    }

    pub fn add_my_project_to_pool(ctx: Context<AddMyProjectToPool>, project_idx: u8) -> ProgramResult {
        let pool = &mut ctx.accounts.in_which_pool.load_mut()?;
        let project = &mut ctx.accounts.project.load_mut()?;

        let my_project = project.my_project[project_idx];

        // pool.

        Ok(())
    }

    // pub fn vote_for_project_in_pool(ctx: Context<VoteForProjectInPool>) -> ProgramResult {
    //     Ok(())
    // }
}

#[derive(Accounts)]
pub struct CreateMatchPool<'info> {
    pub pool_creator: AccountInfo<'info>,
    pub pool: Account<'info, TokenAccount>,
    pub pool_amount_token: Account<'info, TokenAccount>,
    pub pool_account: Box<Account<'info, MatchPool>>,
    pub system_program: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
}

impl<'info> CreateMatchPool<'info> {
    fn into_transfer_to_pda_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self
                .pool_amount_token
                .to_account_info()
                .clone(),
            to: self.pool.to_account_info().clone(),
            authority: self.pool_creator.clone(),
        };
        CpiContext::new(self.token_program.clone(), cpi_accounts)
    }

    fn into_set_authority_context(&self) -> CpiContext<'_, '_, '_, 'info, SetAuthority<'info>> {
        let cpi_accounts = SetAuthority {
            account_or_mint: self.pool.to_account_info().clone(),
            current_authority: self.pool_creator.clone(),
        };
        CpiContext::new(self.token_program.clone(), cpi_accounts)
    }
}

#[derive(Accounts)]
pub struct AddMyProjectToPool<'info> {
    pub project_creator: AccountInfo<'info>,
    pub in_which_pool: Box<Account<'info, MatchPool>>,
    pub project: Box<Account<'info, Project>>,
    pub pool_amount_token: Account<'info, TokenAccount>,
    pub token_program: AccountInfo<'info>,
}

// #[derive(Accounts)]
// pub struct VoteForProjectInPool<'info> {
//     pub voter: AccountInfo<'info>,
// }

#[derive(Accounts)]
pub struct CreateProject<'info> {
    #[account(zero)]
    project: AccountLoader<'info, MyProject>,
}

#[account]
pub struct MatchPool {
    pub pool_creator: Pubkey,
    pub pool_amount_token: Pubkey,
    pub pool_amount: u64,
    pub start_time: i64,
    pub end_time: i64,
}

#[account(zero_copy)]
pub struct MyProject {
    pub my_project: [Project; 256],
}

#[zero_copy]
pub struct Project {
    pub project_id: u8,
    // pub project_owner: Pubkey,
    // // pub project_name: 
    // pub project_amount_by_voters_token: Pubkey,
    pub project_amount_by_voters: u64,
    pub total_votes: u32,
}

// #[account(zero_copy)]
// #[repr(packed)]
// // #[derive(Default)]
// pub struct Voter {
//     pub voter: Pubkey,
//     pub project_voted_to: [Pubkey; 256],
// }
