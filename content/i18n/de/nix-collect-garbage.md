# nix-collect-garbage

Löschen von unbenutzten und unerreichbaren Nix-Speicherpfaden. Generationen können mit `nix-env --list-generations` aufgelistet werden.

## Usage
- `nix-collect-garbage --delete-old` : lösche alle Speicherpfade, die von den aktuellen Generationen der einzelnen Profile nicht verwendet werden
- `nix-collect-garbage --delete-old --dry-run` : simuliere die Löschung alter Speicherpfade
- `nix-collect-garbage --delete-older-than 30d` : lösche alle Speicherpfade, die älter als 30 Tage sind

## Links
- More information: https://nix.dev/manual/nix/stable/command-ref/nix-collect-garbage.html
