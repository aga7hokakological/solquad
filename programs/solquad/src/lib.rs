use anchor_lang::prelude::*;
use anchor_spl::token::{self, SetAuthority, TokenAccount, Transfer, Token};
// use spl_token::instruction::AuthorityType;
use std::iter::Iterator;

declare_id!("5sFUqUTjAMJARrEafMX8f4J1LagdUQ9Y8TR8HwGNHkU8");

#[program]
pub mod solquad {
    use super::*;

    pub fn initialize_escrow(ctx: Context<InitializeEscrow>, amount: u64) -> Result<()> {
        let escrow_account = &mut ctx.accounts.escrow_account;
        escrow_account.escrow_creator = ctx.accounts.escrow_signer.key();
        escrow_account.creator_deposit_amount = amount;
        escrow_account.total_projects = 0;
        // escrow_account.project_reciever_addresses = 
        Ok(())
    }

    pub fn initialize_pool(ctx: Context<InitializePool>) -> Result<()> {
        let pool_account = &mut ctx.accounts.pool_account;
        // pool_account.pool_id = 
        pool_account.pool_creator = ctx.accounts.pool_signer.key();
        pool_account.total_projects = 0;
        pool_account.total_votes = 0;

        Ok(())
    }

    pub fn initialize_project(ctx: Context<InitializeProject>, name: String) -> Result<()> {
        let project_account = &mut ctx.accounts.project_account;

        project_account.project_owner = ctx.accounts.project_owner.key();
        project_account.project_name = name;
        project_account.votes_count = 0;
        project_account.voter_amount = 0;
        project_account.distributed_amt = 0;

        Ok(())
    }

    pub fn add_project_to_pool(ctx: Context<AddProjectToPool>, name: String) -> Result<()> {
        let escrow_account = &mut ctx.accounts.escrow_account;
        let pool_account = &mut ctx.accounts.pool_account;
        let project_account = &mut ctx.accounts.project_account;

        project_account.project_owner = ctx.accounts.project_owner.key();
        project_account.project_name = name;
        project_account.votes_count = 0;
        project_account.voter_amount = 0;
        project_account.distributed_amt = 0;

        // let owner = project_account.project_owner

        // let project = Project {
        //     project_owner: ctx.accounts.project_owner.key(),
        //     project_name: name,
        //     votes_count: 0,
        //     voter_amount: 0,
        //     distributed_amt: 0,
        // };

        pool_account.projects.push(
            ctx.accounts.project_owner.key()
        );
        pool_account.total_projects += 1;

        escrow_account.project_reciever_addresses.push(project_account.project_owner);

        Ok(())
    }

    pub fn vote_for_project(ctx: Context<VoteForProject>, key: Pubkey, amount: u64) -> Result<()> {
        let pool_account = &mut ctx.accounts.pool_account;
        let project_account = &mut ctx.accounts.project_account;
        let voter_account = &mut ctx.accounts.voter_account;

        voter_account.voter = ctx.accounts.voter_sig.key();
        voter_account.voted_for = key;
        voter_account.token_amount = amount;

        project_account.votes_count += 1;
        project_account.voter_amount += amount;

        pool_account.total_votes += 1;

        Ok(())
    }

    // pub fn distribute_escrow_amount(ctx: Context<DistributeEscrowAmount>) -> Result<()> {
    //     let escrow_account = &mut ctx.accounts.escrow_account;
    //     let pool_account = &mut ctx.accounts.pool_account;
    //     let project_account = &mut ctx.accounts.project_account;
  
    //     for i in 0..escrow_account.project_reciever_addresses.len() {
    //         let distributable_amt: u64;
    //         let votes: u16 = pool_account.projects.iter().map(|i| i.votes_count).collect():<Vec<_>>().unwrap();
    //         // let project = pool_account.projects.get(i).unwrap_or(0);
    //         // let votes = project.votes_count;

    //         if votes != 0 {
    //             distributable_amt = (votes / pool_account.total_votes) as u64;
    //         }

    //         project_account.distributed_amt = distributable_amt;
    //     }

    //     Ok(())
    // }
}

#[derive(Accounts)]
pub struct InitializeEscrow<'info> {
    #[account(
        mut,
        seeds = [b"escrow".as_ref(), escrow_signer.key().as_ref()],
        bump,
    )]
    pub escrow_account: Account<'info, Escrow>,
    #[account(mut)]
    pub escrow_signer: Signer<'info>,
    // pub token_account: Account<'info, TokenAccount>,
    // pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializePool<'info> {
    #[account(
        mut,
        seeds = [b"pool".as_ref(), pool_signer.key().as_ref()],
        bump,
    )]
    pub pool_account: Account<'info, Pool>,
    #[account(mut)]
    pub pool_signer: Signer<'info>,
    // pub pool_token_account: Account<'info, TokenAccount>,
    // pub token_program: Program<'info, Token>,
    // pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeProject<'info> {
    #[account(mut)]
    pub project_account: Account<'info, Project>,
    #[account(mut)]
    pub project_owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct AddProjectToPool<'info> {
    #[account(mut)]
    pub escrow_account: Account<'info, Escrow>,
    #[account(mut)]
    pub pool_account: Account<'info, Pool>,
    #[account(mut)]
    pub project_account: Account<'info, Project>,
    #[account(mut)]
    pub project_owner: Signer<'info>,
    // pub pool_token_account: Account<'info, TokenAccount>,
    // pub token_program: Program<'info, Token>,
    // pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct VoteForProject<'info> {
    #[account(mut)]
    pub pool_account: Account<'info, Pool>,
    #[account(mut)]
    pub project_account: Account<'info, Project>,
    #[account(mut)]
    pub voter_account: Account<'info, Voter>,
    #[account(mut)]
    pub voter_sig: Signer<'info>,
    // pub voter_token_account: Account<'info, TokenAccount>,
    // pub token_program: Program<'info, Token>,
    // pub system_program: Program<'info, System>,
}

// #[derive(Accounts)]
// pub struct DistributeEscrowAmount<'info> {
//     #[account(mut)]
//     pub escrow_account: Account<'info, Escrow>,
//     #[account(mut)]
//     pub pool_account: Account<'info, Pool>,
//     #[account(mut)]
//     pub project_account: Account<'info, Project>,
//     #[account(mut)]
//     pub escrow_owner: Signer<'info>,
// }

// Escrow account for quadratic funding
#[account]
pub struct Escrow {
    pub escrow_creator: Pubkey,
    pub creator_deposit_amount: u64,
    pub total_projects: u8,
    pub project_reciever_addresses: Vec<Pubkey>,
    // pub escrow_start_time: i64,
    // pub escrow_end_time: i64,
}

// Pool for each project 
#[account]
pub struct Pool {
    // pub pool_id: u8,
    pub pool_creator: Pubkey,
    // pub projects: Vec<Project>,
    pub projects: Vec<Pubkey>,
    pub total_projects: u8,
    pub total_votes: u64,
}

// Projects in each pool
// #[derive(AnchorSerialize, AnchorDeserialize, Clone)]
#[account]
pub struct Project {
    // pub project_id: u8,
    pub project_owner: Pubkey,
    pub project_name: String,
    pub votes_count: u16,
    pub voter_amount: u64,
    pub distributed_amt: u64,
}

// Voters voting for the project
#[account]
pub struct Voter {
    pub voter: Pubkey,
    pub voted_for: Pubkey,
    pub token_amount: u64
}