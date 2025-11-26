{
  description = "Nix Flake for GTK-RS Development Environment (Nightly)";

  inputs = {
    # Stick to the stable NixOS channel for dependencies like GTK
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";
  };

  outputs = { self, nixpkgs, ... }:
  let
    # Define the native system architecture (Linux build)
    system = "x86_64-linux";
    pkgs = import nixpkgs { inherit system; };

    # Define the cross-compilation system (Windows build)
    crossSystem = {
      system = "x86_64-pc-windows-gnu";
      host = system;
    };

    # --- Nightly Rust Setup: Fix 'rust-bin' Missing Error ---
    # We define an overlay to explicitly pull the pre-compiled nightly toolchain 
    # and use it for the rustPlatform attribute. This resolves the scoping issue.
    rustNightlyOverlay = final: prev: {
      # Use the unstable rust-bin to get the nightly toolchain (required for edition 2024)
      rust = (final.rust-bin.unstable.latest.minimal.override {
        # Ensure the nightly toolchain is compiled with the Windows target enabled
        targets = [ crossSystem.system ];
      });
      # Set the rustPlatform to use the nightly toolchain's platform
      rustPlatform = final.rust.rustPlatform;
    };
    
    # Import nixpkgs with the nightly toolchain applied via the overlay
    nightlyPkgs = import nixpkgs {
      inherit system;
      overlays = [ rustNightlyOverlay ];
      config = {
        allowUnfree = true;
      };
    };

    # The toolchain for the devShell
    nightlyToolchain = nightlyPkgs.rust;


    # Function to create the GTK Rust derivation for a specific system (native or cross)
    mkGtkRsDerivation = { targetSystem, systemPkgs, platformMeta }:
      # IMPORTANT: Use the rustPlatform from the NIGHTLY package set
      nightlyPkgs.rustPlatform.buildRustPackage {
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
  }
}