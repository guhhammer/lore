# gem

Un gestionnaire de paquets pour le langage de programmation Ruby.

## Usage
- `gem search regex -a` : recherche des gems distantes et affiche toutes les versions disponibles
- `gem install nom_gem` : installe la dernière version d'une gem
- `gem install nom_gem -v 1.0.0` : installe une version spécifique d'une gem
- `gem install nom_gem -v '~> 1.0'` : installe la dernière version correspondante (SemVer) d'une gem
- `gem update nom_gem` : mise à jour d'une gem
- `gem list` : liste toutes les gems locales
- `gem uninstall nom_gem` : désinstalle une gem
- `gem uninstall nom_gem -v 1.0.0` : désinstalle une version spécifique d'une gem

## Links
- More information: https://guides.rubygems.org/command-reference/
