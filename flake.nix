{
  description = "Nix Flake for GTK-RS Development Environment (Nightly)";

  inputs = {
    # Stick to the stable NixOS channel for dependencies like GTK
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";
    
    # *** FIX: Use rust-overlay for reliable nightly toolchain access ***
    rust-overlay.url = "github:oxalica/rust-overlay";
    # Ensure rust-overlay uses the same stable channel for consistency
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs"; 
  };

  outputs = { self, nixpkgs, rust-overlay, ... }:
  let
    # Define the native system architecture (Linux build)
    system = "x86_64-linux";
    pkgs = import nixpkgs { inherit system; };

    # Define the cross-compilation system (Windows build)
    crossSystem = {
      system = "x86_64-pc-windows-gnu";
      host = system;
    };

    # --- Nightly Rust Setup using rust-overlay ---
    # 1. Get the toolchain overlay
    rustToolchainOverlay = rust-overlay.overlays.default;

    # 2. Apply the overlay to the stable pkgs set
    rustPkgs = import nixpkgs {
      inherit system;
      overlays = [ rustToolchainOverlay ];
      config = {
        allowUnfree = true; # Required for pre-compiled rust toolchains
      };
    };

    # 3. Get the nightly toolchain package from the new pkgs set
    # .fromRustupToolchain is the reliable way to specify a specific channel like nightly
    nightlyRust = rustPkgs.rustToolchain.fromRustupToolchain {
      toolchain = "nightly";
      # Include the cross-compilation target when defining the toolchain
      targets = [ crossSystem.system ];
    };

    # The rustPlatform we will use for building
    nightlyRustPlatform = nightlyRust.rustPlatform;

    # The toolchain for the devShell
    nightlyToolchain = nightlyRust;
    

    # Function to create the GTK Rust derivation for a specific system (native or cross)
    mkGtkRsDerivation = { targetSystem, systemPkgs, platformMeta }:
      # IMPORTANT: Use the rustPlatform from the NIGHTLY toolchain
      nightlyRustPlatform.buildRustPackage {
        pname = "minimal-gtk-app";
        version = self.rev or "dirty";

        src = self;
        
        # REQUIRED FOR NIX RUST BUILDS
        # You must replace this with the actual hash printed by Nix on the first run!
        cargoHash = "";

        # Add the GTK/Adwaita dependencies needed for building
        buildInputs = with systemPkgs; [
          gtk4
          libadwaita
        ];

        # Add `pkg-config` and C compiler for linking
        nativeBuildInputs = with systemPkgs; [
          pkg-config
          systemPkgs.stdenv.cc
        ];

        # Ensure the build uses the correct cross-compilation target
        cargoBuildFlags = [
          "--target" targetSystem
        ];

        RUST_TARGET = targetSystem;

        checkPhase = "skip";

        meta = {
          description = "Cross-compiled GTK application";
          platforms = platformMeta;
        };
      };

    # Native (Linux) Package
    nativePackage = mkGtkRsDerivation {
      targetSystem = system;
      systemPkgs = pkgs;
      platformMeta = pkgs.lib.platforms.all;
    };

    # Cross-Compiled (Windows) Package
    crossPkgs = import nixpkgs {
      inherit crossSystem;
      localSystem = { inherit system; };
      config = { };
    };

    windowsPackage = mkGtkRsDerivation {
      targetSystem = crossSystem.system;
      systemPkgs = crossPkgs;
      platformMeta = pkgs.lib.platforms.windows; 
    };


  in
  {
    # Defines the development shell environment
    devShells.${system}.default = pkgs.mkShell {
      name = "gtk-rs-development-environment";

      nativeBuildInputs = with pkgs; [
        pkg-config
        gcc
        rust-analyzer
      ];
      
      # Use the nightly tools for the shell itself
      buildInputs = [
        nightlyToolchain.rustc
        nightlyToolchain.cargo
        # Standard GTK dependencies
        pkgs.gtk4
        pkgs.libadwaita
        pkgs.xorg.libX11
        # Dev dependencies
        pkgs.gobject-introspection
        pkgs.libadwaita.dev
        pkgs.glib.dev
        pkgs.gtk4.dev
        pkgs.xorg.libX11.dev
      ];

      TMPDIR = "/tmp";

      shellHook = ''
        export RUST_BACKTRACE=1
        echo "--------------------------------------------------------"
        echo "GTK-RS development environment ready (NIGHTLY Rust)!"
        echo "--------------------------------------------------------"
      '';
    };

    # Export packages for building
    packages.${system} = {
      default = nativePackage;
      minimal-gtk-app = nativePackage;
    };

    packages.${crossSystem.system} = {
      default = windowsPackage;
      minimal-gtk-app = windowsPackage;
    };
  };
}