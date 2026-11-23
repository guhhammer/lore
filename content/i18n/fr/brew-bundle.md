# brew bundle

Gestionnaire de paquets pour Homebrew, Homebrew Cask et le Mac App Store.

## Usage
- `brew bundle` : installe tous les paquets listés dans le Brewfile situé dans le dossier courant
- `brew bundle --file chemin/vers/fichier` : installe tous les paquets listés dans le Brewfile situé au chemin spécifié
- `brew bundle dump` : créé un Brewfile avec tous les paquets installés actuellement
- `brew bundle cleanup --force` : désinstalle tous les paquets non listés dans le Brewfile
- `brew bundle check` : vérifie si il y a un ou plusieurs paquets à installer ou à mettre à jour depuis le Brewfile
- `brew bundle list --all` : affiche la liste de toutes les entrées dans un Brewfile

## Links
- More information: https://docs.brew.sh/Manpage#bundle-subcommand
