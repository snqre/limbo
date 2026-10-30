{ ... }: {
	perSystem = { pkgs, system, ... }: {
		packages.anchor =
		(let
			pname = "anchor";
			version = "1.2.2";
			asset."x86_64-linux".name = "anchor-${version}-x86_64-unknown-linux-gnu";
			asset."x86_64-linux".hash = "e2c35a973bcfe476f3a89b120e99820f917229faf33d3e71f6625bd94199e741";
			asset."x86_64-darwin".name = "anchor-${version}-x86_64-apple-darwin";
			asset."x86_64-darwin".hash = "bd5a0c4504512f07e19bb86a70abe80a8a054b2562773d8b6a1a0f8b0407332d";
			asset."aarch64-darwin".name = "anchor-${version}-aarch64-apple-darwin";
			asset."aarch64-darwin".hash = "fbfa9dc5c130dde033970aad20271ca10f1d20e9abcb78506c29b6c1684cc7d1";
			configuration = (asset.${system} or (throw "unsupported (system=${system})"));
		in (pkgs.stdenv.mkDerivation {
			inherit pname;
			inherit version;

			src = (pkgs.fetchurl {
				url = "https://github.com/otter-sec/anchor/releases/download/v${version}/${configuration.name}";
				sha256 = configuration.hash;
			});

			dontUnpack = true;

			nativeBuildInputs = pkgs.lib.optional pkgs.stdenv.isLinux pkgs.autoPatchelfHook;

			buildInputs = [
				pkgs.openssl
				pkgs.udev
				pkgs.zlib
			] ++ pkgs.lib.optionals pkgs.stdenv.hostPlatform.isLinux [
				pkgs.stdenv.cc.cc.lib
			];

			installPhase = ''
				install -Dm755 $src $out/bin/anchor
			'';

			meta.description = "Solana Program Framework CLI";
			meta.license = pkgs.lib.licenses.asl20;
			meta.platforms = [
				"x86_64-linux"
				"x86_64-darwin"
				"aarch64-darwin"
			];
		}));
	};
}
