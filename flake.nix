{
  description = "Nix Flake for GTK-RS Development Environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";
  };

  outputs = { self, nixpkgs, ... }:
  let
    # Define the native system architecture (Linux build)
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};

    # Define the cross-compilation system (Windows build)
    crossSystem = {
      # Target for MinGW (Windows)
      system = "x86_64-pc-windows-gnu";
      # The host system where the cross-compiler runs
      host = system;
    };

    # Function to create the GTK Rust derivation for a specific system (native or cross)
    mkGtkRsDerivation = { targetSystem, systemPkgs, platformMeta }:
      # Use `rustPlatform.buildRustPackage` for reliable Rust compilation
      systemPkgs.rustPlatform.buildRustPackage {
        pname = "minimal-gtk-app";
        version = self.rev or "dirty";

        # The source code is the current directory
        src = self;
        
        # REQUIRED FOR NIX RUST BUILDS: Needs a hash of the Cargo.lock dependencies
        # Use an empty string hash for the first build. Nix will tell you the correct hash to use.
        cargoHash = "";

        # Add the GTK/Adwaita dependencies needed for building
        buildInputs = with systemPkgs; [
          # GTK/GLib dependencies (will pull MinGW versions when cross-compiling)
          gtk4
          libadwaita
        ];

        # Add `pkg-config` and C compiler for linking
        nativeBuildInputs = with systemPkgs; [
          pkg-config
          # The appropriate cross-compiler (gcc) will be automatically selected by Nixpkgs
          systemPkgs.stdenv.cc
        ];

        # Ensure the build uses the correct cross-compilation target
        cargoBuildFlags = [
          "--target" targetSystem
        ];

        # Set the target triplet explicitly for the linker
        RUST_TARGET = targetSystem;

        # Standard checks and hardening
        checkPhase = ''
          # Optional: Run tests for the target (usually disabled for cross-compilation)
          # cargo test --target $RUST_TARGET
        '';

        # Standard check for all derivations
        meta = {
          description = "Cross-compiled GTK application";
          # Use the platformMeta passed to the function
          platforms = platformMeta;
        };
      };

    # Native (Linux) Package
    nativePackage = mkGtkRsDerivation {
      targetSystem = system;
      systemPkgs = pkgs;
      platformMeta = pkgs.lib.platforms.all; # Use all for native build
    };

    # Cross-Compiled (Windows) Package
    crossPkgs = import nixpkgs {
      inherit crossSystem;
      # We still use the native system as the host for the build tools
      localSystem = { inherit system; };
      config = { }; # Standard config
    };

    windowsPackage = mkGtkRsDerivation {
      targetSystem = crossSystem.system;
      systemPkgs = crossPkgs;
      # Explicitly use Nixpkgs windows platforms list for the cross-compiled target
      platformMeta = pkgs.lib.platforms.windows; 
    };


  in
  {
    # Defines the development shell environment (Only for native development on Linux)
    devShells.${system}.default = pkgs.mkShell {
      name = "gtk-rs-development-environment";

      # Tools/Compilers needed for the build process (added rust-analyzer for IDE support)
      nativeBuildInputs = with pkgs; [
        pkg-config
        gcc

        # Declarative Rust toolchain (Replaces rustup/cargo from your original shell)
        rustc
        cargo
        rust-analyzer

        # Dependencies for GTK-RS/GNOME development
        gobject-introspection
        libadwaita.dev
        glib.dev
        gtk4.dev
        xorg.libX11.dev
      ];

      # Libraries needed to run the resulting application
      buildInputs = with pkgs; [
        gtk4
        libadwaita
        xorg.libX11
      ];

      # Setting TMPDIR as requested
      TMPDIR = "/tmp";

      # Environment setup hook
      shellHook = ''
        export RUST_BACKTRACE=1
        echo "--------------------------------------------------------"
        echo "GTK-RS development environment ready (Flake-based)!"
        echo "Use 'cargo build' or 'nix build' to build the native Linux app."
        echo "--------------------------------------------------------"
      '';
    };

    # Export packages for building (The 'nix build' targets)
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