


pub fn admin_redeem(ctx: Context<AdminRedeem>, redeem_amount: u64) -> Result<()> {
    let state = &mut ctx.accounts.global_state;
    require!(!state.paused, CustomError::ContractPaused);
    require!(ctx.accounts.admin.key() == state.admin, CustomError::Unauthorized);
    require!(redeem_amount <= state.total_minted, CustomError::ExceedMintedSupply);
    
    let released_collateral = redeem_amount * state.reserve_ratio / 100;
    require!(ctx.accounts.vault_token_account.amount >= released_collateral, CustomError::InsufficientVaultCollateral);
    
    // Borrow ctx.accounts immutably for burn context
    let burn_ctx = ctx.accounts.into_burn_context();
    token::burn(burn_ctx, redeem_amount)?;

    // Borrow ctx.accounts immutably for transfer context
    let transfer_ctx = ctx.accounts.into_return_context().with_signer(&[&[b"vault", &[ctx.bumps.vault_authority]]]);
    token::transfer(
        transfer_ctx,
        released_collateral
    )?;
    
    // Now modify the state after the immutable borrows have been released
    state.total_minted = state.total_minted.saturating_sub(redeem_amount);
    state.total_collateral = state.total_collateral.saturating_sub(released_collateral);
    
    emit!(CollateralRedeemed { amount: released_collateral });
    Ok(())
}
