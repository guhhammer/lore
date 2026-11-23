# nix develop

Lance un terminal Bash avec l'environnement de construction de la dérivation.

## Usage
- `nix develop nixpkgs#pkg` : lance un terminal Bash avec toutes les dépendances d'un paquet disponible dans nixpkgs
- `nix develop` : lance un terminal de développement pour le paquet par défaut dans un flake du répertoire actuel
- `configurePhase; buildPhase` : configure et construit les sources dans le terminal lancé

## Links
- More information: https://nix.dev/manual/nix/stable/command-ref/new-cli/nix3-develop.html
