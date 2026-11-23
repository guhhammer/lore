# nix-build.3

Construit une expression Nix (télécharge depuis le cache si possible).

## Usage
- `nix build nixpkgs#pkg` : construit un paquet depuis nixpkgs, en liant le résultat symboliquement à `./result`
- `nix build -L .#pkg` : construit un paquet depuis un flake du répertoire actuel, en affichant les journaux du processus
- `nix build chemin/vers/répertoire` : construit le paquet par défaut d'un flake depuis un répertoire quelconque
- `nix build --no-link --print-out-paths` : construit un paquet sans lier symboliquement le résultat, mais affiche le chemin de stockage dans le `stdout`

## Links
- More information: https://nix.dev/manual/nix/stable/command-ref/new-cli/nix3-build.html
