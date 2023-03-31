{
  description = "Advent of Code solutions by SemPruijs";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = { self, nixpkgs, flake-parts, ... }: flake-parts.lib.mkFlake { inherit self; } {
    systems = [ "x86_64-linux" "aarch64-linux" "aarch64-darwin" "x86_64-darwin" ];
    perSystem = { config, self', inputs', pkgs, system, ... }: {
      devShells = {
        default =
          pkgs.mkShell {
            buildInputs = with pkgs; [ nodePackages.typescript nodePackages.typescript-language-server nodePackages.ts-node nodejs bacon rnix-lsp hyperfine cargo-flamegraph cargo];
          };
      };
    };
  };
}
