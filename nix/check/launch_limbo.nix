{ ... }: {
	perSystem = ({ pkgs, config, ... }: {
		checks.launch_limbo = (pkgs.testers.runNixOSTest {
			name = "main";
			skipLint = true;
			skipTypeCheck = true;

			nodes.main.environment.systemPackages = [
				pkgs.anchor-cli
				pkgs.solana-cli
				pkgs.jq
				pkgs.python3

				config.packages.limbo
				config.packages.limbo_client
				config.packages.spl_token_cli
			];

			nodes.main.networking.firewall.enable = false;

			testScript = (pkgs.lib.replaceStrings [
				"@LIMBO_SO@"
				"@LIMBO_PROGRAM_KEYPAIR@"
			] [
				"${config.packages.limbo}/limbo.so"
				"${config.packages.limbo}/limbo-keypair.json"
			]
			(builtins.readFile ./launch_limbo.script.py));
		});
	});
}
