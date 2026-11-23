# brew

Gestionnaire de paquets pour macOS et Linux. Certaines sous-commandes, telles que `install`, disposent de leur propre documentation d'utilisation.

## Usage
- `brew install formule|cask` : installe la dernière version stable d'une formule ou d'un cask
- `brew list` : liste toutes les formules et les casks installés
- `brew upgrade formule|cask` : met à jour une formule ou cask déjà installé (si rien n'est précisé, toutes les formules et tous les casks installés seront mis à jour)
- `brew update` : récupère la dernière version d'Homebrew et toutes les formules et casks depuis le dépôt source d'Homebrew
- `brew outdated` : montre les formules et les casks disposants d'une nouvelle version
- `brew search texte` : recherche une formule (c.a.d. un package) et/ou un cask (c.a.d une application native)
- `brew info formule|cask` : affiche les informations d'une formule ou d'un cask (version, chemin d'installation, dépendances, etc.)
- `brew doctor` : vérifie que l'installation locale d'Homebrew n'a pas de problème

## Links
- More information: https://docs.brew.sh/Manpage
