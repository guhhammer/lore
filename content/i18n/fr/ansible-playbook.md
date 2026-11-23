# ansible-playbook

Exécute les tâches définies dans le playbook sur les machines distantes via SSH.

## Usage
- `ansible-playbook playbook` : exécute les tâches d'un playbook
- `ansible-playbook playbook -i fichier_d_inventaire` : exécute les tâches d'un playbook avec fichier d'inventaire spécifié
- `ansible-playbook playbook -e "variable1=valeur1 variable2=valeur2"` : exécute les tâches d'un playbook avec des variables supplémentaires définies via la ligne de commande
- `ansible-playbook playbook -e "@variables.json"` : exécute les tâches d'un playbook avec des variables supplémentaires définies dans un fichier JSON
- `ansible-playbook playbook -t tag1,tag2` : exécute les tâches d'un playbook pour certain tags
- `ansible-playbook playbook --start-at nom_de_la_tache` : exécute les tâches d'un playbook en démarrant depuis une certaine tache

## Links
- More information: https://docs.ansible.com/projects/ansible/latest/cli/ansible-playbook.html
