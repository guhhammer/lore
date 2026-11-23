# vagrant plugin

Gère les plugiciels Vagrant.

## Usage
- `vagrant plugin list` : liste tous les plugiciels actuellement installés
- `vagrant plugin install vagrant_vbguest` : installe un plugiciel depuis des dépôts distants, généralement RubyGems
- `vagrant plugin install chemin/vers/plugiciel.gem` : installe un plugiciel à partir d’un fichier local
- `vagrant plugin update` : met à jour tous les plugiciels installés vers leur dernière version
- `vagrant plugin update vagrant_vbguest` : met à jour un plugiciel à la dernière version
- `vagrant plugin uninstall vagrant_vbguest` : désinstalle un plugiciel spécifique

## Links
- More information: https://developer.hashicorp.com/vagrant/docs/cli/plugin
