use anchor_lang::prelude::*;

declare_id!("2Jq4CX5Eez8cPJfLeKDwFdpvuuyjqZhzo1p1TL3tUbbR");

type Result<T = ()> = anchor_lang::Result<T>;

#[derive(Accounts)]
struct New<'a> {
	pub system_program: Program<'a, System>,

	#[account(mut)]
	pub creator: Signer<'a>
}

#[program]
mod contract {
	use super::*;

	pub fn new(context: Context<New>, amount: u64) -> Result {

		Ok(())
	}
}
