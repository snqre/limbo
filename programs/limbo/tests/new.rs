use litesvm::*;
use solana_sdk::instruction::*;
use solana_sdk::pubkey::*;
use solana_sdk::message::*;
use solana_sdk::signature::*;
use solana_sdk::transaction::*;
use solana_address::*;

#[test]
fn new() {
	let exit: std::process::ExitStatus = std::process::Command::new("anchor").args([
		"build",
		"--",
		"--package", "limbo"
	])
	.current_dir("../../")
	.stdout(std::process::Stdio::inherit())
	.stderr(std::process::Stdio::inherit())
	.status()
	.expect("anchor build execution");

	assert!(exit.success(), "anchor build failed");

	let mut svm: LiteSVM = LiteSVM::default();

	let program_pk: Pubkey = Pubkey::new_unique();
	let program_pk_bytes: [_; _] = program_pk.to_bytes();
	let program_addr: Address = program_pk_bytes.into();
	let buyer_keypair: Keypair = Keypair::new();
	let buyer_pk: Pubkey = buyer_keypair.pubkey();
	let buyer_pk_bytes: [_; _] = buyer_pk.to_bytes();
	let buyer_addr: Address = buyer_pk_bytes.into();

	svm.add_program_from_file(program_addr, "../../target/deploy/limbo.so").expect("`.so`");
	svm.airdrop(&buyer_addr, 10_000_000_000).expect("airdrop");
}
