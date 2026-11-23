# vagrant

Gère des environnements de développement légers, reproductibles et portables. Certaines sous-commandes, telles que `box`, `snapshot`, `halt`, etc., disposent de leur propre documentation d’utilisation.

## Usage
- `vagrant init` : crée un fichier Vagrantfile dans le répertoire actuel avec l'image Vagrant de base
- `vagrant init ubuntu/focal64` : crée un fichier Vagrantfile avec une image depuis le Vagrant Public Registry
- `vagrant up` : démarre et configure l’environnement Vagrant
- `vagrant suspend` : suspend la machine
- `vagrant halt` : arrête la machine
- `vagrant ssh` : se connecte à la machine via SSH
- `vagrant ssh-config` : affiche la configuration SSH de la machine Vagrant en cours d’exécution
- `vagrant box list` : liste toutes les images locales

## Links
- More information: https://developer.hashicorp.com/vagrant/docs/cli
