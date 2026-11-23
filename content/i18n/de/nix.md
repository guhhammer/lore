# nix

Ein leistungsfähiger Paketmanager, der das Paketmanagement zuverlässig, reproduzierbar und deklarativ macht. `nix` ist experimentell und muss gesondert aktiviert werden. Einige Unterbefehle wie `build`, `develop`, `flake`, `registry`, `profile`, `search`, `repl`, `store`, `edit`, `why-depends` usw. haben ihre eigene Dokumentation.

## Usage
- `mkdir -p ~/.config/nix; echo 'experimental-features = nix-command flakes' >~/.config/nix/nix.conf` : aktiviere den `nix` Befehl
- `nix search nixpkgs suchbegriff` : suche ein Paket in nixpkgs nach Namen oder Beschreibung
- `nix shell nixpkgs#pkg1 nixpkgs#pkg2 nixpkgs#pkg3 ...` : starte eine Shell und stelle die angegebenen Pakete von nixpkgs darin bereit
- `nix profile install nixpkgs#pkg1 nixpkgs#pkg2 nixpkgs#pkg3 ...` : installiere einige Pakete von nixpkgs dauerhaft
- `nix store gc` : entferne ungenutzte Pfade aus dem Nix-Store, um Speicherplatz freizugeben
- `nix repl` : starte eine interaktive Umgebung zur Auswertung von Nix-Ausdrücken
- `nix help unterbefehl` : zeige Hilfe für einen bestimmten Unterbefehl an

## Links
- More information: https://nix.dev/manual/nix/stable/command-ref/new-cli/nix
