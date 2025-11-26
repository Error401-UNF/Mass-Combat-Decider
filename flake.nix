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
    # Target system KEY required by GitHub Action for output attribute
    windowsSystem = "x86_64-pc-windows-gnu"; 
    # GCC Triplet KEY for cross-compilation target
    windowsGccTriplet = "x86_64-w64-mingw32"; 

    # ---------------------------------------------------------
    # Package Sets
    # ---------------------------------------------------------
    
    # 1. Native Linux Pkgs (Clean version for devShell and native build)
    pkgsLinuxNative = nixpkgs.legacyPackages.${linuxSystem};

    # 2. Cross-Compilation Pkgs (Linux Host -> Windows Target)
    # FIX: Isolate the cross-compilation import to prevent configuration leakage and 
    # solve the 'attribute missing' error, which occurs because MinGW isn't in 
    # the default pkgsCross list for this channel.
    pkgsCrossWindows = import nixpkgs {
      system = linuxSystem; # The machine that runs the build (GitHub Runner)
      crossSystem = {
        config = windowsGccTriplet; # The target machine architecture (Windows MinGW)
      };
      config = {
        # Allow packages marked as "unsupported" (like libxkbcommon)
        allowUnsupportedSystem = true;
        # Allow packages marked as "broken" (like Python3)
        allowBroken = true;
      };
    };

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
      # pkgs will be pkgsLinuxNative for Linux build, and pkgsCrossWindows for Windows build.
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