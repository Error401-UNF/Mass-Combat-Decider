{
  description = "A local development shell for Rust Project 1";

  inputs = {
    # Pin Nixpkgs to a specific commit/branch for reproducibility
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
  };

  outputs = { self, nixpkgs, ... }:
  let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in
  {
    devShells.${system}.default = pkgs.mkShell {
      name = "rust-project-1-dev";

      nativeBuildInputs = with pkgs; [
        pkg-config

        rust-analyzer
        cargo
        rustc
        gcc

        # GTK dependencies
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
        sqlite
      ];

      shellHook = ''
        unset RUSTFLAGS
        unset LDFLAGS
        unset CFLAGS
        
        export RUST_BACKTRACE=1
        echo "--------------------------------------------------------"
        echo "Rust Mass Combat Desider Environment Ready."
        echo "--------------------------------------------------------"
      '';
    };
  };
}
