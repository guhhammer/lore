# nix-env

Manipulieren oder Abfragen von Nix-Benutzerumgebungen.

## Usage
- `nix-env -q` : liste alle installierten Pakete auf
- `nix-env -q suchbegriff` : frage installierte Pakete ab
- `nix-env -qa suchbegriff` : frage verfügbare Pakete ab
- `nix-env -iA nixpkgs.paket_name` : installiere Paket
- `nix-env -i paket_name --file example.com` : installiere ein Paket von einer URL
- `nix-env -e paket_name` : deinstalliere ein Paket
- `nix-env -u paket_name` : upgrade ein Pakets
- `nix-env -u` : upgrade alle Pakete

## Links
- More information: https://nix.dev/manual/nix/stable/command-ref/nix-env.html
