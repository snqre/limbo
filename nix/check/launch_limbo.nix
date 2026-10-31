{ ... }: {
	perSystem = ({ pkgs, config, ... }: {
		checks.launch_limbo = (pkgs.testers.runNixOSTest {
			name = "main";

			nodes.main.environment.systemPackages = [
				pkgs.anchor-cli
				pkgs.solana-cli
				pkgs.jq
				pkgs.python3

				config.packages.limbo
				config.packages.spl_token_cli
			];

			nodes.main.networking.firewall.enable = false;

			testScript = (pkgs.lib.replaceStrings [
				"@LIMBO_SO@"
			] [
				"${config.packages.limbo}/limbo.so"
			]
			(builtins.readFile ./launch_limbo.script.py));
		});
	});
}
