# nix-shell.2

Startet eine interaktive Shell, die auf einem Nix-Ausdruck basiert.

## Usage
- `nix-shell` : starte mit Nix-Ausdruck in `shell.nix` oder `default.nix` im aktuellen Verzeichnis
- `nix-shell --run "befehl argument1 argument2 ..."` : führe Shell-Befehl in nicht-interaktiver Shell aus und beende
- `nix-shell default.nix` : starte mit Ausdruck in `default.nix` im aktuellen Verzeichnis
- `nix-shell -p paket_name_1 paket_name_2 ...` : starte mit aus nixpkgs geladenen Paketen
- `nix-shell -p paket_name_1 paket_name_2 ... -I nixpkgs=https://github.com/NixOS/nixpkgs/archive/nixpkgs_revision.tar.gz` : starte mit Paketen, die aus einer bestimmten Nixpkgs-Revision geladen wurden
- `nix-shell -i interpreter -p paket_name_1 paket_name_2 ...` : führe den Rest der Datei mit einem bestimmten Interpreter aus, zur Verwendung in `#!-scripts` (siehe <https://nixos.org/manual/nix/stable/#use-as-a-interpreter>)

## Links
- More information: https://nix.dev/manual/nix/stable/command-ref/nix-shell.html
