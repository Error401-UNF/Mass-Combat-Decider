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
    # The workflow specifically asks for this system key for Windows
    windowsSystem = "x86_64-pc-windows-gnu";

    # ---------------------------------------------------------
    # Package Sets
    # ---------------------------------------------------------
    
    # 1. Native Linux Pkgs (For devShell and Linux build)
    pkgsLinux = nixpkgs.legacyPackages.${linuxSystem};

    # 2. Cross-Compilation Pkgs (Linux Host -> Windows Target)
    # We configure this to build ON Linux (system) FOR Windows (crossSystem)
    pkgsCrossWindows = import nixpkgs {
      system = linuxSystem;
      crossSystem = {
        config = "x86_64-w64-mingw32";
      };
      # FIX: Allow packages marked as "not for Windows" (like libxkbcommon) to attempt building anyway.
      # This is often necessary for GTK/Adwaita cross-compilation.
      config = {
        allowUnsupportedSystem = true;
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

      # Native Build Inputs (Tools needed at build time)
      nativeBuildInputs = with pkgs; [
        pkg-config
      ] ++ pkgs.lib.optionals pkgs.stdenv.isLinux [
        # wrapGAppsHook4 is strictly for Linux to help find schemas/icons
        wrapGAppsHook4
      ];

      # Build Inputs (Libraries linked into the binary)
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
      default = mkApp pkgsLinux;
    };

    # 2. Windows Output (Cross Compiled)
    # The workflow requests: .#packages.x86_64-pc-windows-gnu.default
    packages.${windowsSystem} = {
      default = mkApp pkgsCrossWindows;
    };

    # ---------------------------------------------------------
    # Outputs: DevShell (Preserved from your original file)
    # ---------------------------------------------------------
    devShells.${linuxSystem}.default = pkgsLinux.mkShell {
      name = "gtk-rs-development-environment";

      nativeBuildInputs = with pkgsLinux; [
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

      buildInputs = with pkgsLinux; [
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