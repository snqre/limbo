{ ... }: {
	perSystem = { pkgs, system, ... }: {
		packages.solana_cli =
		(let
			version = "1.18.18";

			system_to_selection."x86_64-linux".arch = "x86_64-unknown-linux-gnu";
			system_to_selection."x86_64-linux".hash = pkgs.lib.fakeHash;

			configuration = system_to_selection.${system} or (throw "unsupported (system=${system})");
		in pkgs.stdenv.mkDerivation {
			inherit version;

			pname = "solana_cli";

			src = pkgs.fetchurl {
				url = "https://github.com/anza-xyz/agave/releases/download/v${version}/solana-release-${configuration.arch}.tar.bz2";
				sha256 = pkgs.lib.fakeHash;
			};

			nativeBuildInputs = [
				pkgs.autoPatchelfHook
				pkgs.makeWrapper
			];

			buildInputs = [
				pkgs.stdenv.cc.cc.lib
				pkgs.zlib
				pkgs.openssl
				pkgs.udev
			];

			installPhase = ''
				mkdir -p $out

				cp -r . $out/
			'';
		});
	};
}
