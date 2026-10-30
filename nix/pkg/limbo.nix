{ self, ... }: {
	perSystem = { pkgs, config, ... }: {
		packages.limbo =
		(let
			pname = "limbo";
			version = "0.1.0";
			src = self;
			secret_key = (builtins.getEnv "SOLANA_SECRET_KEY");
		in (pkgs.lib.mkForce (pkgs.stdenv.mkDerivation {
			inherit pname;
			inherit version;
			inherit src;

			__noChroot = true;

			SSL_CERT_FILE = "${pkgs.cacert}/etc/ssl/certs/ca-bundle.crt";

			nativeBuildInputs = [
				pkgs.nushell

				config.rust-project.toolchain
				config.packages.anchor
				config.packages.solana
			];

			buildPhase = '' nu -c '
				$env.HOME = ($env.TMPDIR | path join "home")

				mkdir $env.HOME
				mkdir target/deploy

				"${secret_key}" | save target/deploy/limbo-keypair.json

				anchor build --arch sbfv1
			' '';

			installPhase = '' nu -c '
				mkdir -p $out/deploy

				cp -r target/deploy/* $out/deploy/
			' '';
		})));
	};
}
