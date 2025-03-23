use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer, MintTo, Burn};
declare_id!("4t9PtdHj8aedrXA4F3etpDM268kAQYbP2LTxnXUECtj3");

#[program]
pub mod stablecoin_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn initialize_global_state(ctx: Context<InitializeGlobalState>, mint_cap: u64, reserve_ratio: u64) -> Result<()> {
        require!(mint_cap > 0, CustomError::InvalidParameter);
        require!(reserve_ratio <= 100, CustomError::InvalidParameter);
        
        
        
        let (vault_pda, _) = Pubkey::find_program_address(&[b"vault"], ctx.program_id);
        let (mint_auth_pda, _) = Pubkey::find_program_address(&[b"mint", ctx.accounts.global_state.key().as_ref()], ctx.program_id);
        
        
        
        let state = &mut ctx.accounts.global_state;
        state.admin = ctx.accounts.admin.key();
        state.vault_authority = vault_pda;
        state.mint_authority = mint_auth_pda;
        state.total_collateral = 0;
        state.total_minted = 0;
        state.mint_cap = mint_cap;
        state.reserve_ratio = reserve_ratio;
        state.paused = false;
        
        emit!(SystemInitialized { admin: state.admin });
        Ok(())
    }
    
    
    
    pub fn update_mint_cap(ctx: Context<AdminOnly>, new_cap: u64) -> Result<()> {
        ctx.accounts.global_state.mint_cap = new_cap;
        Ok(())
    }
    
    
    
    pub fn update_reserve_ratio(ctx: Context<AdminOnly>, new_ratio: u64) -> Result<()> {
        ctx.accounts.global_state.reserve_ratio = new_ratio;
        Ok(())
    }
    
    
    
    pub fn emergency_pause(ctx: Context<AdminOnly>, status: bool) -> Result<()> {
        ctx.accounts.global_state.paused = status;
        Ok(())
    }
    
    
    
    pub fn deposit_collateral(ctx: Context<DepositCollateral>, amount: u64) -> Result<()> {
        require!(!ctx.accounts.global_state.paused, CustomError::ContractPaused);
        token::transfer(ctx.accounts.into_transfer_context(), amount)?;
        ctx.accounts.global_state.total_collateral = ctx.accounts.global_state.total_collateral.saturating_add(amount);
        emit!(CollateralDeposited { user: ctx.accounts.user.key(), amount });
        Ok(())
    }
    
    
    
    // pub fn admin_mint_stablecoin(ctx: Context<MintStablecoin>, mint_amount: u64) -> Result<()> {
    //     let state = &mut ctx.accounts.global_state;
    //     require!(!state.paused, CustomError::ContractPaused);
    //     require!(ctx.accounts.user.key() == state.admin, CustomError::Unauthorized);
    //     require!(mint_amount <= state.mint_cap.saturating_sub(state.total_minted), CustomError::ExceedMintCap);
        
        
        
    //     let required_collateral = (state.total_minted.saturating_add(mint_amount)) * state.reserve_ratio / 100;
    //     require!(state.total_collateral >= required_collateral, CustomError::InsufficientGlobalReserve);
        
    //     let global_state_key = state.key();
    //     token::mint_to(
    //         ctx.accounts.into_mint_context().with_signer(&[&[b"mint", global_state_key.as_ref(), &[ctx.bumps.mint_authority_bump]]]),
    //         mint_amount,
    //     )?;
        
        
        
    //     state.total_minted = state.total_minted.saturating_add(mint_amount);
    //     emit!(StablecoinMinted { amount: mint_amount });
    //     Ok(())
    // }
    
    
    
    pub fn admin_redeem(ctx: Context<AdminRedeem>, redeem_amount: u64) -> Result<()> {
        let burn_ctx = ctx.accounts.into_burn_context();
        let binding = &[b"vault", &[ctx.bumps.vault_authority]];
        let transfer_ctx = ctx.accounts.into_return_context().with_signer(&binding);
        let state = &mut ctx.accounts.global_state;
        require!(!state.paused, CustomError::ContractPaused);
        require!(ctx.accounts.admin.key() == state.admin, CustomError::Unauthorized);
        require!(redeem_amount <= state.total_minted, CustomError::ExceedMintedSupply);
             
        let released_collateral = redeem_amount * state.reserve_ratio / 100;
        require!(ctx.accounts.vault_token_account.amount >= released_collateral, CustomError::InsufficientVaultCollateral);
        
        token::burn(burn_ctx, redeem_amount)?;
        
        token::transfer(
            transfer_ctx,
            released_collateral
        )?;
        
        state.total_minted = state.total_minted.saturating_sub(redeem_amount);
        state.total_collateral = state.total_collateral.saturating_sub(released_collateral);
        
        emit!(CollateralRedeemed { amount: released_collateral });
        Ok(())
    }
}
    
    
    
#[derive(Accounts)]
pub struct InitializeGlobalState<'info> {
    ///CHECK:
    #[account(init, payer = admin, space = 8 + 104)]
    pub global_state: Account<'info, GlobalState>,
    #[account(mut)] pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}
    
    
    
#[derive(Accounts)]
pub struct AdminOnly<'info> {
    #[account(mut, has_one = admin)]
    pub global_state: Account<'info, GlobalState>,
    pub admin: Signer<'info>,
}
    
    
    
#[derive(Accounts)]
pub struct DepositCollateral<'info> {
    #[account(mut)] pub user: Signer<'info>,
    #[account(mut, constraint = user_token_account.mint == vault_token_account.mint @ CustomError::InvalidCollateralToken)]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(mut)] pub vault_token_account: Account<'info, TokenAccount>,
    #[account(mut)] pub global_state: Account<'info, GlobalState>,
    pub token_program: Program<'info, Token>,
}
    
    
    
#[derive(Accounts)]
pub struct MintStablecoin<'info> {
    #[account(mut)] pub user: Signer<'info>,
    #[account(mut)] pub stablecoin_mint: Account<'info, Mint>,
    #[account(mut)] pub user_stablecoin_account: Account<'info, TokenAccount>,
    #[account(mut)] pub mint_authority: UncheckedAccount<'info>,
    #[account(mut)] pub global_state: Account<'info, GlobalState>,
    pub token_program: Program<'info, Token>,
    /// CHECK: This is safe because we use it as a PDA
    #[account(seeds = [b"mint", global_state.key().as_ref()], bump)]
    pub mint_authority_bump: UncheckedAccount<'info>,
}
    
    
    
#[derive(Accounts)]
pub struct AdminRedeem<'info> {
    #[account(mut)] pub admin: Signer<'info>,
    #[account(mut)] pub stablecoin_mint: Account<'info, Mint>,
    #[account(mut)] pub admin_stablecoin_account: Account<'info, TokenAccount>,
    #[account(mut)] pub vault_token_account: Account<'info, TokenAccount>,
    #[account(mut)] pub admin_collateral_account: Account<'info, TokenAccount>,
    #[account(seeds = [b"vault"], bump)] pub vault_authority: UncheckedAccount<'info>,
    #[account(mut)] pub global_state: Account<'info, GlobalState>,
    pub token_program: Program<'info, Token>,
}
    
    
    
#[account]
#[derive(InitSpace)]
pub struct GlobalState {
    pub admin: Pubkey,
    pub vault_authority: Pubkey,
    pub mint_authority: Pubkey,
    pub total_collateral: u64,
    pub total_minted: u64,
    pub mint_cap: u64,
    pub reserve_ratio: u64,
    pub paused: bool,
}
    
    
    
#[error_code]
pub enum CustomError {
    #[msg("Insufficient global reserve to mint stablecoin")] InsufficientGlobalReserve,
    #[msg("Exceeded global mint cap")] ExceedMintCap,
    #[msg("Unauthorized admin access")] Unauthorized,
    #[msg("Contract is paused")] ContractPaused,
    #[msg("Invalid initialization parameter")] InvalidParameter,
    #[msg("Vault collateral not enough for redeem")] InsufficientVaultCollateral,
    #[msg("Invalid collateral token type")] InvalidCollateralToken,
    #[msg("Exceed total minted supply")] ExceedMintedSupply,
}
    
    
    
#[event]
pub struct CollateralDeposited {
    pub user: Pubkey,
    pub amount: u64,
}
    
    
    
#[event]
pub struct StablecoinMinted {
    pub amount: u64,
}
    
    
    
#[event]
pub struct CollateralRedeemed {
    pub amount: u64,
}
    
    
    
#[event]
pub struct SystemInitialized {
    pub admin: Pubkey,
}
    
    
    
impl<'info> DepositCollateral<'info> {
    fn into_transfer_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.user_token_account.to_account_info(),
                to: self.vault_token_account.to_account_info(),
                authority: self.user.to_account_info(),
            },
        )
        }
}
    
    
    
impl<'info> MintStablecoin<'info> {
    fn into_mint_context(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            MintTo {
                        mint: self.stablecoin_mint.to_account_info(),
                        to: self.user_stablecoin_account.to_account_info(),
                        authority: self.mint_authority.to_account_info(),
                    },)
        }
}
    
    
    
impl<'info> AdminRedeem<'info> {
    fn into_burn_context(&self) -> CpiContext<'_, '_, '_, 'info, Burn<'info>> {
        CpiContext::new(
        self.token_program.to_account_info(),
        Burn {
                    mint: self.stablecoin_mint.to_account_info(),
                    from: self.admin_stablecoin_account.to_account_info(),
                    authority: self.admin.to_account_info(),
                },
                )
    }

    fn into_return_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
        self.token_program.to_account_info(),
        Transfer {
                    from: self.vault_token_account.to_account_info(),
                    to: self.admin_collateral_account.to_account_info(),
                    authority: self.vault_authority.to_account_info(),
                },
        )
    }
}
    
#[derive(Accounts)]
pub struct Initialize {}