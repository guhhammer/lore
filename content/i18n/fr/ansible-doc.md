# ansible-doc

Affiche les informations des modules installés dans les librairies Ansible. Affiche une liste concise des plugins et leurs description courte.

## Usage
- `ansible-doc -l` : liste les plugins action (modules) disponibles
- `ansible-doc -t type_de_plugin -l` : liste les plugins disponible d'un certain type
- `ansible-doc nom_du_plugin` : affiche les informations sur un plugin action (module) spécifique
- `ansible-doc -t type_de_plugin nom_du_plugin` : affiche les informations sur un plugin avec un certain type
- `ansible-doc -s nom_du_plugin` : affiche le raccourci playbook d'un plugin action (module)
- `ansible-doc -j nom_du_plugin` : affiche les informations sur un plugin action (module) en JSON

## Links
- More information: https://docs.ansible.com/projects/ansible/latest/cli/ansible-doc.html
