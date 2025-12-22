use super::*;

/// Function for casting votes
///
/// Arguements:-
///     - Transaction Hash
///     - can_transact can be either true or false
///  
/// Fails when:-
///     - caller is not from validator set
///     - voter is same
///
/// Emits cast vote event
pub fn cast_vote(ctx: Context<CastVotes>, tx_hash: String, can_transact: bool) -> Result<()> {
    let validators = &ctx.accounts.validators;
    let votes = &mut ctx.accounts.votes;
    let threshold = &ctx.accounts.threshold;
    let caller = ctx.accounts.authority.to_account_info().key;

    // Ensuring authorized sender
    require!(
        validators.addresses.contains(caller),
        CustomError::Unauthorized
    );

    let mut vote = Votes::new(can_transact, vec![*caller]);

    // Ensuring not the same voter
    require!(
        !votes.voters.contains(caller),
        CustomError::PermissionDenied {}
    );

    votes.update(&mut vote);
    if votes.yes >= threshold.value {
        votes.set_status(Status::Ready);
    }

    // Emit cast vote event
    emit!(CastVoteEvent {
        tx_hash,
        can_transact
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(tx_hash: String)]
pub struct CastVotes<'info> {
    #[account(
        seeds = [VALIDATORS_TAG],
        bump,
    )]
    pub validators: Account<'info, Validators>,

    #[account(
        seeds = [THRESHOLD_TAG],
        bump,
    )]
    pub threshold: Account<'info, Threshold>,

    #[account(
        init,
        seeds = [&tx_hash.as_bytes()[(tx_hash.len() - 32)..tx_hash.len()]],
        bump,
        payer = authority,
        space = std::mem::size_of::<Votes>() + 32
    )]
    pub votes: Account<'info, Votes>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}
