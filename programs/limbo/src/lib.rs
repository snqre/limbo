use anchor_lang::prelude::*;

declare_id!("2Jq4CX5Eez8cPJfLeKDwFdpvuuyjqZhzo1p1TL3tUbbR");

type Result<T = ()> = anchor_lang::Result<T>;

#[derive(Accounts)]
struct New<'info> {
	pub system_program: Program<'info, System>,

	#[account(mut)]
	pub creator: Signer<'info>
}

#[program]
mod contract {
	use super::*;

	pub fn new(context: Context<New>, amount: u64) -> Result {

		Ok(())
	}
}
