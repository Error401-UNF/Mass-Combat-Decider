{
  description = "Nix Flake for GTK-RS Development and Bundling";

  inputs = {
    # Pin to nixpkgs-unstable for the latest GTK4 packages
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = { self, nixpkgs, ... }:
  let
    # System definition
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};

    # Dependencies required by the GTK application
    gtkDeps = with pkgs; [ gtk4 libadwaita glib xorg.libX11 ];

    # 1. The core Rust package (built with standard rustPlatform)
    massCombatDecider = pkgs.rustPlatform.buildRustPackage {
      pname = "mass-combat-decider";
      version = "0.1.1";
      
      # Use the special 'self' attribute to reference the directory containing flake.nix
      src = self; 
      
      # Assuming Cargo.lock is in the same directory as flake.nix
      cargoLock = { lockFile = ./Cargo.lock; }; 

      # Standard build inputs
      nativeBuildInputs = with pkgs; [ pkg-config ];
      buildInputs = gtkDeps;
      
      # Name of the executable (derived from the crate name)
      cargoBuildFlags = [ "--bin MassCombatDecider" ];
  
      # This tells Nix to remove debug symbols from the final binary
      dontStrip = false;
    };

    # 2. The final bundled package (creates the AppImage-like wrapper)
    bundledApp = pkgs.stdenv.mkDerivation {
      pname = "mass-combat-decider-portable";
      version = "0.1.1";
      
      src = massCombatDecider; 

      # Tools needed to create the wrapper MUST be here
      nativeBuildInputs = [ pkgs.makeWrapper ]; 
      buildInputs = gtkDeps;

      # We change installPhase to a phases-conscious block or fix the hook usage
      installPhase = ''
        runHook preInstall

        localName="mass-combat-decider-portable"
        mkdir -p $out/bin
        
        cp $src/bin/MassCombatDecider $out/bin/$localName
        chmod +x $out/bin/$localName
        
        wrapProgram $out/bin/$localName \
          --prefix PATH : ${pkgs.lib.makeBinPath gtkDeps} \
          --prefix LD_LIBRARY_PATH : ${pkgs.lib.makeLibraryPath gtkDeps}

        runHook postInstall
      '';
    };

  in
  {
    # ---------------------------------------------------------
    # FIX: Add defaultPackage attribute for nix-appimage bundler
    # ---------------------------------------------------------
    defaultPackage.${system} = bundledApp;
    
    # ---------------------------------------------------------
    # Outputs: Packages (Bundled Linux App)
    # ---------------------------------------------------------
    packages.${system}.default = bundledApp;

    # ---------------------------------------------------------
    # Outputs: DevShell
    # ---------------------------------------------------------
    devShells.${system}.default = pkgs.mkShell {
      name = "gtk-rs-development-environment";

      nativeBuildInputs = with pkgs; [
        pkg-config gcc rustc cargo rust-analyzer gobject-introspection
        libadwaita.dev glib.dev gtk4.dev xorg.libX11.dev
      ];

      buildInputs = gtkDeps;

      shellHook = ''
        export RUST_BACKTRACE=1
        echo "--------------------------------------------------------"
        echo "GTK-RS development environment ready (Flake-based)!"
        echo "Use 'cargo run' to build and run your GTK application."
        echo "Use 'nix build' to create the bundled portable app."
        echo "--------------------------------------------------------"
      '';
    };
  };
}