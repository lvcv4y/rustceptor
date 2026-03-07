{
  description = ''
    Flake for rustceptor. Allows to access the built version and
      the developing tools from Nix package manager.
  '';

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=25.11";

    # Some flake utilities.
    flake-utils.url = "github:numtide/flake-utils";

    # Rust builder ; we'll use crane, so trunk build is available.
    crane.url = "github:ipetkov/crane";

    # Nix-pure Rust compiling toolchain.
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };


    # wasm-bindgen-cli must have the same version as the cargo.lock one.
    nixpkgs-wasm = {
      url = "github:nixos/nixpkgs/80d901ec0377e19ac3f7bb8c035201e2e098cc97";
    };

    # tailwindcss-animate ; needed as node module.
    tailwindcss-animate = {
      url = "github:jamiebuilds/tailwindcss-animate?ref=v1.0.7";
      # hash = "sha256-ipg8fCXqDgrNo3uoIGNFOR5ip6j5Ak0CsdquhjDOefs=";
      flake = false;
    };
  };

  outputs = {
    nixpkgs,
    flake-utils,
    crane,
    rust-overlay,
    nixpkgs-wasm,
    tailwindcss-animate,
    ... 
    }: flake-utils.lib.eachDefaultSystem ( system:  # Generate flake for each architecture.
    let 
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ (import rust-overlay) ]; # Override toolchain
      };

      inherit (pkgs) lib;

      /*****************
       *     Builds    *
       *****************/
      
      /*
       * Backend
       */

      # Initialize crane with rust toolchain
      craneLibBack = crane.mkLib pkgs;

      # Build
      backend = craneLibBack.buildPackage {
        src = craneLibBack.cleanCargoSource (craneLibBack.path ./backend/.);
        strictDeps = true;

        # Custom environment variables can be passed here
        # *without* rebuilding all dependency crates
        # ENVIRONMENT = "hello";

        # buildInputs = [
        #   # Add additional build inputs here
        # ]
      };

      /*
       * Frontend (Trunk build)
       */
      rustToolchainFor = p:
        p.rust-bin.stable.latest.default.override {
          # Set the build targets supported by the toolchain,
          # wasm32-unknown-unknown is required for trunk
          targets = [ "wasm32-unknown-unknown" ];
        };

      craneLibFront = (crane.mkLib pkgs).overrideToolchain rustToolchainFor;

      frontSrc =
        let
          unfilteredRoot = ./frontend/.;
        in lib.fileset.toSource {
          root = unfilteredRoot;
          fileset = lib.fileset.unions [
            # Default files from crane (Rust and cargo files)
            (craneLibFront.fileset.commonCargoSources unfilteredRoot)
            (lib.fileset.fileFilter (
              file:
              lib.any file.hasExt [
                "html"
                "scss"
                "css"
                "js"
                "json"
              ]
            ) unfilteredRoot)

            # Example of a folder for images, icons, etc
            # (lib.fileset.maybeMissing ./assets)
          ];
        };

      frontCommonArgs = {
        src = frontSrc;
        strictDeps = true;
        # We must force the target, otherwise cargo will attempt to use your native target
        CARGO_BUILD_TARGET = "wasm32-unknown-unknown";

        buildInputs = [
          # Additional dependencies
          # pkgs.tailwindcss  # Already in Trunk
        ];
      };

      frontend = craneLibFront.buildTrunkPackage (frontCommonArgs // {
        cargoArtifacts = craneLibFront.buildDepsOnly(frontCommonArgs // {
          # You cannot run cargo test on a wasm build
          doCheck = false;
        });

        # Gotta match the Cargo.lock wasm-bindgen version
        wasm-bindgen-cli = nixpkgs-wasm.legacyPackages.${system}.wasm-bindgen-cli;

        preBuild =
          # Somewhat needed, Trunk has some trouble finding where to build
          "export HOME=$TMPDIR;\n"
          +

          # tailwindcss-animate isn't provided by Trunk, and hasn't its dedicated nix package.
          # So, we mock a node_modules folder, with the repo content inside, so that trunk is happy.
          ''
            mkdir -p node_modules/tailwindcss-animate
            cp -r ${tailwindcss-animate}/* node_modules/tailwindcss-animate/
          '';
      });

      /*****************
       * Dev commands  *
       *****************/

      backend-cmd = pkgs.writeShellScriptBin "backend" ''
        export RUST_LOG=debug;
        export ENV=debug;
        ${backend}/bin/backend
      '';

      backend-front-cmd = pkgs.writeShellScriptBin "backwfront" ''
        export RUST_LOG=debug;
        export ENV=debug;
        export FRONT_PATH=${frontend};
        ${backend}/bin/backend
      '';

      # That's not really the purest way of doing this, but that'll do for now.
      frontend-cmd = pkgs.writeShellScriptBin "frontend" ''
        RUST_LOG=debug;
        ENV=debug;
        (cd ./frontend && ${pkgs.trunk}/bin/trunk serve)
      '';
    
    in {
      packages = {
        inherit frontend backend;
      };

      apps = {
        frontend = {
          type = "app";
          program = "${frontend-cmd}/bin/frontend";
        };

        backend = {
          type = "app";
          program = "${backend-cmd}/bin/backend";
        };

        backwfront = {
          type = "app";
          program = "${backend-front-cmd}/bin/backwfront";
        };
      };
    }
  );
}