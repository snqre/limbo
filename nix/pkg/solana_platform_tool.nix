{ ... }: {
	perSystem = { pkgs, system, ... }: {
		packages.solana_platform_tool =
		(let
			pname = "solana_platform_tool";
			version = "v1.41";

			asset."x86_64-linux".hash = pkgs.lib.fakeHash;

			configuration = (asset.${system} or (throw "unsupported (system=${system})"));
		in (pkgs.stdenv.mkDerivation {
			inherit pname;
			inherit version;

			src = (pkgs.fetchurl {
				url = "https://github.com/anza-xyz/platform-tools/releases/download/${version}/platform-tools-linux-x86_64.tar.bz2";
				hash = configuration.hash;
			});

			nativeBuildInputs = [
				pkgs.autoPatchelfHook
			];

			buildInputs = [
				pkgs.stdenv.cc.cc.lib
				pkgs.zlib
				pkgs.zlib-ng
				pkgs.openssl
			];

			installPhase = ''
				mkdir -p $out

				cp -r . $out/
			'';
		}));
	};
}
