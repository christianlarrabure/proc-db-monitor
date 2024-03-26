# https://ayats.org/blog/nix-workflow/
{
  description = "A flake for Untold Dawn MUD Development";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { self, nixpkgs }: let 
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    devShells.${system}.default = pkgs.mkShell {
      packages = [
        pkgs.rustup
        # If the dependencies need system libs, you usually need pkg-config + the lib
        pkgs.pkg-config
      ];

      env = {
        RUST_BACKTRACE = "full";
      };
  
    };

  };
}
