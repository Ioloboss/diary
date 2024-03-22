{
	description = "Rust Diary App";

	inputs = {
		nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
	};

	outputs = { self, nixpkgs, ... }:
		let
			allSystems = [ "x86_64-linux" ];
			forAllSystems = f: nixpkgs.lib.genAttrs allSystems (system: f {
    				pkgs = import nixpkgs { inherit system; };
			});
		in
		{
    			packages = forAllSystems ({ pkgs }: {
        			default =
        				let
        					buildInputs = with pkgs; [];
        				in
        					pkgs.rustPlatform.buildRustPackage {
							name = "diary";
							version = "0.1.0";
							src = self;
							inherit buildInputs;
							cargoLock.lockFile = ./Cargo.lock;
							allowSubstitutes = false;
        					};
    			});
		};
}
