{ self, ... }: {
	perSystem = { pkgs, config, ... }: {
		packages.limbo =
		(let
			pname = "limbo";
			version = "0.1.0";
			src = self;
			rust_platform = (pkgs.makeRustPlatform {
				rustc = config.rust-project.toolchain;
				cargo = config.rust-project.toolchain;
			});
		in (pkgs.lib.mkForce (pkgs.stdenv.mkDerivation {
			__noChroot = true;

			SSL_CERT_FILE = "${pkgs.cacert}/etc/ssl/certs/ca-bundle.crt";

			nativeBuildInputs = [
				pkgs.nushell

				config.rust-project.toolchain
				config.anchor
			];

			buildPhase = '' nu -c '
				$env.HOME = ($env.TMPDIR | path join "home")

				mkdir $env.HOME
				mkdir $env.out


			' '';
		})));
	};
}
