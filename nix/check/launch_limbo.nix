{ ... }: {
	perSystem = ({ pkgs, config, ... }: {
		checks.launch_limbo = (pkgs.testers.runNixOSTest {
			name = "main";

			nodes.main.environment.systemPackages = [
				config.packages.limbo
			];

			testScript = ''
start_all()

main.wait_for_unit("multi-user.target")

main.succeed("ls -la ${config.packages.limbo}/deploy")
main.succeed("[ -f ${config.packages.limbo}/deploy/limbo.so ] || [ -f ${config.packages.limbo}/deploy/limbo_program.so ]")
			'';
		});
	});
}
