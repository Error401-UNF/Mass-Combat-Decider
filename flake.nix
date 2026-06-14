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
    gtkDeps = with pkgs; [ gtk4 libadwaita glib xorg.libX11 ];

    # 1. The core Rust package (built with standard rustPlatform)
    massCombatDecider = pkgs.rustPlatform.buildRustPackage {
      pname = "MassCombatDecider";
      version = "0.1.3";
      src = self; 
      cargoLock = { lockFile = ./Cargo.lock; };
      nativeBuildInputs = with pkgs; [ pkg-config ];
      buildInputs = gtkDeps;
      cargoBuildFlags = [ "--bin MassCombatDecider" ];
      dontStrip = false;
    };

    # 2. The final bundled package (creates the AppImage-like wrapper)
    bundledApp = pkgs.stdenv.mkDerivation {
      pname = "MassCombatDecider";
      version = "0.1.3";
      src = massCombatDecider; 
      nativeBuildInputs = [ pkgs.makeWrapper ]; 
      buildInputs = gtkDeps;
      installPhase = ''
        runHook preInstall

        localName="MassCombatDecider"
        mkdir -p $out/bin
        
        cp $src/bin/MassCombatDecider $out/bin/$localName
        chmod +x $out/bin/$localName
        
        wrapProgram $out/bin/$localName \
          --prefix PATH : ${pkgs.lib.makeBinPath gtkDeps} \
          --prefix LD_LIBRARY_PATH : ${pkgs.lib.makeLibraryPath gtkDeps}
        # --- Create Desktop Entry for Launchers (Rofi, etc.) ---
        mkdir -p $out/share/applications
        cat > $out/share/applications/MassCombatDecider.desktop <<EOF
[Desktop Entry]
Type=Application
Name=Mass Combat Decider
Comment=Simulate large-scale D&D combat efficiently
Exec=$out/bin/MassCombatDecider
Icon=system-run
Terminal=false
Categories=Game;Utility;
EOF
        runHook postInstall
      '';
    };
  in
  {
    defaultPackage.${system} = bundledApp;
    packages.${system}.default = bundledApp;
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