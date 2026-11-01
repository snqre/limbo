use litesvm::*;
use solana_sdk::instruction::*;
use solana_sdk::pubkey::*;
use solana_sdk::message::*;
use solana_sdk::signature::*;
use solana_sdk::transaction::*;
use solana_address::*;

#[test]
fn new() -> eyre::Result {
	let local_root_directory: std::path::PathBuf = env!("CARGO_MANIFEST_DIR").to_owned().into();

	let workspace_root_directory: &std::path::Path = local_root_directory
		.parent()
		.ok_or(eyre::eyre!("no parent"))?
		.parent()
		.ok_or(eyre::eyre!("no parent"))?;

	let limbo_so_path: std::path::PathBuf = workspace_root_directory
    	.join("target")
     	.join("sbpfv3-solana-solana")
      	.join("release")
       	.join("limbo.so");

	let exit: std::process::ExitStatus = std::process::Command::new("anchor").args([
		"build",
		"--",
		"--package", "limbo"
	])
	.current_dir(workspace_root_directory)
	.stdout(std::process::Stdio::inherit())
	.stderr(std::process::Stdio::inherit())
	.status()?;

	assert!(exit.success(), "anchor build failed");

	let mut svm: LiteSVM = LiteSVM::new();

	let program_pk: Pubkey = Pubkey::new_unique();
	let program_pk_bytes: [_; _] = program_pk.to_bytes();
	let program_addr: Address = program_pk_bytes.into();
	let buyer_keypair: Keypair = Keypair::new();
	let buyer_pk: Pubkey = buyer_keypair.pubkey();
	let buyer_pk_bytes: [_; _] = buyer_pk.to_bytes();
	let buyer_addr: Address = buyer_pk_bytes.into();

	svm.add_program_from_file(program_addr, limbo_so_path)?;
	svm.airdrop(&buyer_pk, 10_000_000_000).expect("airdrop");

	Ok(())
}
