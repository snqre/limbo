{ ... }: {
	perSystem = { pkgs, ... }: (let
		pname = "spl-token-cli";
		version = "5.6.0";
	in {
		packages.spl_token_cli = (pkgs.rustPlatform.buildRustPackage {
			inherit pname;
			inherit version;

			src = (pkgs.fetchCrate {
				inherit pname;
				inherit version;

				hash = "sha256-4RiUs0a6+uOEuXlLLJaKrcePsPtztlcz0JXDlcb/pHA==";
			});

			cargoHash = "sha256-OXeqwPANQpCM2knGpNnBzdJP5l7gGVJz+FrpxIXSnQ0==";

			doCheck = false;

			nativeBuildInputs = [
				pkgs.pkg-config
				pkgs.perl
				pkgs.rustPlatform.bindgenHook
			];

			buildInputs = [
				pkgs.openssl
			] ++ pkgs.lib.optionals pkgs.stdenv.isLinux [
				pkgs.udev
			];
		});
	});
}
