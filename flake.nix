{
  description = "Nix Flake for GTK-RS Development Environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
  };

  outputs = { self, nixpkgs, ... }:
  let
    # ---------------------------------------------------------
    # System Definitions
    # ---------------------------------------------------------
    linuxSystem = "x86_64-linux";
    # Target platform for MinGW cross-compilation, required by GitHub Action
    windowsSystem = "x86_64-pc-windows-gnu"; 

    # ---------------------------------------------------------
    # Package Sets
    # ---------------------------------------------------------
    
    # 1a. Native Linux Pkgs (Clean version for devShell and native build)
    pkgsLinuxNative = nixpkgs.legacyPackages.${linuxSystem};

    # 1b. Configured Host Pkgs (Linux set with overrides for cross-compilation)
    # We create a new Linux package set here to apply the overrides (broken/unsupported).
    # This prevents the configuration leakage error ('ipc_rmid_deferred_release') 
    # that happens when the flags are mixed with 'crossSystem' in a single import.
    pkgsLinuxConfigured = import nixpkgs {
      system = linuxSystem;
      config = {
        allowUnsupportedSystem = true;
        allowBroken = true;
      };
    };

    # 2. Cross-Compilation Pkgs (Linux Host -> Windows Target)
    # We use the built-in 'pkgsCross' mechanism from the configured host set. This 
    # method ensures a much cleaner evaluation environment for the Windows target.
    pkgsCrossWindows = pkgsLinuxConfigured.pkgsCross.${windowsSystem};

    # ---------------------------------------------------------
    # Build Logic (Reusable)
    # ---------------------------------------------------------
    mkApp = pkgs: pkgs.rustPlatform.buildRustPackage {
      pname = "gtk-app"; 
      version = "0.1.0";
      
      # Use the current directory as source
      src = ./.;

      # Nix requires the Cargo.lock to build rust packages deterministically
      cargoLock = {
        lockFile = ./Cargo.lock;
      };

      # Native Build Inputs (Tools needed at build time, run on HOST: Linux)
      nativeBuildInputs = with pkgs; [
        pkg-config
      ] ++ pkgs.lib.optionals pkgs.stdenv.isLinux [
        # wrapGAppsHook4 is strictly for Linux to help find schemas/icons
        wrapGAppsHook4
      ];

      # Build Inputs (Libraries linked into the binary, for TARGET: Windows or Linux)
      buildInputs = with pkgs; [
        gtk4
        libadwaita
        glib
      ] ++ pkgs.lib.optionals pkgs.stdenv.isLinux [
        # X11 is usually only needed for Linux targets
        xorg.libX11
      ];
      
      # Disable tests during cross-compilation because we can't run Windows exes on Linux
      doCheck = if pkgs.stdenv.hostPlatform != pkgs.stdenv.buildPlatform then false else true;
    };

  in
  {
    # ---------------------------------------------------------
    # Outputs: Packages (Used by GitHub Actions)
    # ---------------------------------------------------------
    
    # 1. Linux Output
    packages.${linuxSystem} = {
      default = mkApp pkgsLinuxNative;
    };

    # 2. Windows Output (Cross Compiled)
    # The workflow requests: .#packages.x86_64-pc-windows-gnu.default
    packages.${windowsSystem} = {
      default = mkApp pkgsCrossWindows;
    };

    # ---------------------------------------------------------
    # Outputs: DevShell (Preserved from your original file)
    # ---------------------------------------------------------
    devShells.${linuxSystem}.default = pkgsLinuxNative.mkShell { # Use the clean native package set
      name = "gtk-rs-development-environment";

      nativeBuildInputs = with pkgsLinuxNative; [
        pkg-config
        gcc
        rustc
        cargo
        rust-analyzer
        gobject-introspection
        libadwaita.dev
        glib.dev
        gtk4.dev
        xorg.libX11.dev
      ];

      buildInputs = with pkgsLinuxNative; [
        gtk4
        libadwaita
        xorg.libX11
      ];

      TMPDIR = "/tmp";

      shellHook = ''
        export RUST_BACKTRACE=1
        echo "--------------------------------------------------------"
        echo "GTK-RS development environment ready (Flake-based)!"
        echo "Use 'cargo run' to build and run your GTK application."
        echo "--------------------------------------------------------"
      '';
    };
  };
}