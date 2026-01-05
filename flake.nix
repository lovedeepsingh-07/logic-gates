{
  description = "logic-gates";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/d2ed99647a4b195f0bcc440f76edfa10aeb3b743";
    rust-overlay = {
      url = "github:oxalica/rust-overlay/a9c35d6e7cb70c5719170b6c2d3bb589c5e048af";
      inputs.nixpkgs.follows = "nixpkgs";
    };
	nixgl = {
		url = "github:nix-community/nixGL/b6105297e6f0cd041670c3e8628394d4ee247ed5";
		inputs.nixpkgs.follows = "nixpkgs";
	};
    flake-utils.url = "github:numtide/flake-utils/11707dc2f618dd54ca8739b309ec4fc024de578b";
  };
  outputs = { ... }@inputs:
    inputs.flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [ (import inputs.rust-overlay) inputs.nixgl.overlay ];
        };
        rust-pkg = pkgs.rust-bin.stable."1.88.0".default;
        deps = import ./deps.nix { inherit pkgs; };
      in
      {
        formatter = pkgs.nixfmt-classic;
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = [ pkgs.ninja pkgs.cmake rust-pkg pkgs.pkg-config pkgs.flatbuffers pkgs.clang ];
          buildInputs = [
		    pkgs.nixgl.nixGLIntel
			pkgs.alsa-lib
			pkgs.libGL
            pkgs.xorg.libX11
            pkgs.libxkbcommon
            pkgs.xorg.libXrandr
            pkgs.xorg.libXinerama
            pkgs.xorg.libXcursor
            pkgs.xorg.libXi
		  ];
		  LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
		  LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath [
            pkgs.xorg.libX11
            pkgs.libxkbcommon
            pkgs.xorg.libXi
		  ]}:$LD_LIBRARY_PATH";
		  shellHook = ''zsh'';
        };
		apps.setup = inputs.flake-utils.lib.mkApp {
		  drv = pkgs.writeShellApplication {
			name = "setup";
			runtimeInputs = [ ];
			text = deps.setup_script;
		  };
		};
      });
}
