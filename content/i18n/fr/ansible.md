# ansible

Gestionnaire de groupes d'ordinateurs à distance depuis SSH. (Utiliser le fichier `/etc/ansible/hosts` pour ajouter de nouveaux groupes/hôtes). Certaines commandes comme `ansible galaxy` ont leur propre documentation.

## Usage
- `ansible groupe --list-hosts` : liste les hôtes appartenant à un groupe
- `ansible groupe -m ping` : ping d'un groupe d'hôtes en invoquant le module "ping"
- `ansible groupe -m setup` : affiche des informations sur un groupe d'hôtes en invoquant le module "setup"
- `ansible groupe -m command -a 'ma_commande'` : exécute une commande sur un groupe d'hôtes en invoquant le module "command" avec en paramètre (a) cette commande
- `ansible groupe -b --ask-become-pass -m command -a 'ma_commande'` : exécute une commande avec des droits administrateur
- `ansible groupe -i fichier_d'inventaire -m command -a 'ma_commande'` : exécute une commande en utilisant un fichier d'inventaire personnalisé
- `ansible localhost -m debug -a 'var=groups.keys()'` : liste les groupes d'un inventaire

## Links
- More information: https://docs.ansible.com/projects/ansible/latest/cli/ansible.html
