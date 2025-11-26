{
  description = "Nix Flake for GTK-RS Development Environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
  };

  outputs = { self, nixpkgs, ... }:
  let
    # Define the system architecture
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in
  {
    # Defines the development shell environment
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
        echo "Use 'cargo run' to build and run your GTK application."
        echo "--------------------------------------------------------"
      '';
    };
  };
}
