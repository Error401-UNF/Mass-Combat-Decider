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
      
      # Use the output of the built Rust package as the source for the wrapper
      src = massCombatDecider; 

      # The standard installation directory
      installPhase = ''
        localName="mass-combat-decider-portable" # Target name for the AppImage bundler
        
        mkdir -p $out/bin
        
        # 1. Copy the built executable to the expected name
        cp $src/bin/MassCombatDecider $out/bin/$localName
        
        # 2. Ensure the copied file is executable (crucial for the AppImage bundler)
        chmod +x $out/bin/$localName
        
        # 3. Create the wrapper script with all GTK dependencies
        wrapProgram $out/bin/$localName \
          --prefix PATH : ${pkgs.lib.makeBinPath gtkDeps} \
          --prefix LD_LIBRARY_PATH : ${pkgs.lib.makeLibraryPath gtkDeps}
      '';

      # Set dependencies so Nix knows what to bundle with the wrapper
      buildInputs = gtkDeps;
      
      # Tools needed to create the wrapper
      nativeBuildInputs = with pkgs; [ makeWrapper ]; 
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