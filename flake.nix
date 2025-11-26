{
  description = "Nix Flake for GTK-RS Development Environment (Linux Native Only)";

  inputs = {
    # Using nixpkgs-unstable for the latest fixes
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = { self, nixpkgs, ... }:
  let
    # ---------------------------------------------------------
    # System Definitions
    # ---------------------------------------------------------
    linuxSystem = "x86_64-linux";
    
    # ---------------------------------------------------------
    # Package Sets
    # ---------------------------------------------------------
    
    # Native Linux Pkgs (The only package set we are using now)
    pkgs = nixpkgs.legacyPackages.${linuxSystem};

    # ---------------------------------------------------------
    # Build Logic (The final application package)
    # ---------------------------------------------------------
    appPackage = pkgs.rustPlatform.buildRustPackage {
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
        wrapGAppsHook4 # For finding GTK schemas/icons on Linux
      ];

      # Build Inputs (Libraries linked into the binary, for TARGET: Linux)
      buildInputs = with pkgs; [
        gtk4
        libadwaita
        glib
        xorg.libX11
      ];
      
      # Enable checks since we are running natively
      doCheck = true;
    };

  in
  {
    # ---------------------------------------------------------
    # Outputs: Packages
    # ---------------------------------------------------------
    
    # Linux Output (The default package)
    packages.${linuxSystem}.default = appPackage;

    # ---------------------------------------------------------
    # Outputs: DevShell
    # ---------------------------------------------------------
    devShells.${linuxSystem}.default = pkgs.mkShell {
      name = "gtk-rs-development-environment";

      nativeBuildInputs = with pkgs; [
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

      buildInputs = with pkgs; [
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