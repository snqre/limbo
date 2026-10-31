{ ... }: {
	perSystem = ({ pkgs, config, ... }: {
		checks.launch_limbo = (pkgs.testers.runNixOSTest {
			name = "main";

			nodes.main.environment.systemPackages = [
				pkgs.anchor-cli
				pkgs.solana-cli

				config.packages.limbo
			];

			nodes.main.networking.firewall.enable = false;

			testScript = ''
start_all()

main.wait_for_unit("multi-user.target")

main.execute("solana-test-validator --reset --quiet > /tmp/validator.log 2>&1 &")

main.wait_for_open_port(8899)

main.succeed("solana cluster-version --url http://127.0.0.1:8899")
main.succeed("solana-keygen new --no-passphrase --outfile /root/.config/solana/id.json")
main.succeed("solana airdrop 10 --url http://127.0.0.1:8899")
main.succeed("solana program deploy ${config.packages.limbo}/limbo.so --url http://127.0.0.1:8899")
			'';
		});
	});
}
