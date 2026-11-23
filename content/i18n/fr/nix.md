# nix

Un puissant gestionnaire de paquets qui rend la gestion des paquets fiable, reproductible et déclarative. `nix` est expérimental et doit être activé avec les fonctionnalités expérimentales. Certaines sous-commandes comme `build`, `develop`, `flake`, `registry`, `profile`, `search`, `repl`, `store`, `edit`, `why-depends`, etc., ont leur propre documentation d'utilisation.

## Usage
- `mkdir -p ~/.config/nix; echo 'experimental-features = nix-command flakes' >~/.config/nix/nix.conf` : active la commande `nix `
- `nix search nixpkgs terme_recherché` : recherche un paquet dans `nixpkgs` par son nom ou sa description
- `nix shell nixpkgs#pkg1 nixpkgs#pkg2 nixpkgs#pkg3 ...` : lance un terminal avec les paquets spécifiés disponibles depuis `nixpkgs`
- `nix profile install nixpkgs#pkg1 nixpkgs#pkg2 nixpkgs#pkg3 ...` : installe des paquets depuis `nixpkgs` de manière permanente
- `nix store gc` : nettoie les chemins inutilisés dans l'espace de stockage de Nix (Nix store) pour libérer de l'espace
- `nix repl` : lance un environnement interactif pour évaluer des expressions Nix
- `nix help sous-commande` : affiche l'aide pour une sous-commande spécifique

## Links
- More information: https://nix.dev/manual/nix/stable/command-ref/new-cli/nix
