{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  name = "gtk-rs-development-environment";

  # Native dependencies needed to compile gtk-rs applications.
  nativeBuildInputs = with pkgs; [
    pkg-config
    rustup
    cargo
    gcc
    # These are crucial for gtk-rs development
    gobject-introspection
    libadwaita.dev
    glib.dev
    # Also add the dev packages for other buildInputs
    gtk4.dev
    xorg.libX11.dev

  ];

  # Libraries needed to run gtk-rs applications.
  buildInputs = with pkgs; [
    gtk4
    libadwaita
    xorg.libX11
  ];

  # Directly set the TMPDIR attribute for mkShell
  TMPDIR = "/tmp";

  shellHook = ''
    export RUST_BACKTRACE=1
    export PATH="$HOME/.cargo/bin:$PATH"
    echo "GTK-RS development environment ready!"
    echo "Use 'cargo run' to build and run your GTK application."
  '';
}
