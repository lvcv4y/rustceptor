{
  description = ''
    An example deployment flake. It takes this repo flake as an input
      to access to the compiled resources.  
  '';

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=25.11";

    rustceptor.url = "github:lvcv4y/rustceptor";
  };

  outputs = {
    self,
    nixpkgs,
    rustceptor,
    ...
  }:
  let
    system = "x86_64-linux"; # Your system architecture ; might be something else.  
  in {
    nixosConfiguration.main = nixpkgs.lib.nixosSystem {
      inherit system;
      specialArgs = {
        # Passes rustceptor as an argument to the building functions.
        # Now the compiled projects are available with rustceptor.backend and rustceptor.frontend
        rustceptor = rustceptor.packages.${system};
      };

      modules = [
        # The actual deployment.
        ./configuration.nix
      ];
    };
  };
}