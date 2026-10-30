use clap::Parser as _;
use solana_sdk::signature::Signer as _;

type Result<T = ()> = eyre::Result<T>;

#[derive(Debug, clap::Subcommand)]
enum SystemHook {
	Welcome
}

#[derive(Debug, clap::Parser)]
enum Command {
	#[clap(subcommand, hide = true)]
	SystemHook(SystemHook),

	GenerateKeyPair
}

fn main() -> Result {
	match Command::parse() {
		Command::SystemHook(SystemHook::Welcome) => Ok(()),
		Command::GenerateKeyPair => {
			let key_pair: solana_sdk::signature::Keypair = solana_sdk::signature::Keypair::new();
			let public_key: solana_sdk::pubkey::Pubkey = key_pair.pubkey();

			todo!()
		}
	}
}
