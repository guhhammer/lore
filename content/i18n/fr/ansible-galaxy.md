# ansible-galaxy

Crée et gère les rôles Ansible.

## Usage
- `ansible-galaxy rôle|collection list` : liste les rôles ou les collections installés
- `ansible-galaxy role search mot_clé -vvvvvv` : recherche un rôle avec différents niveaux de verbosité (`-v` doit être spécifié à la fin)
- `ansible-galaxy role install|remove nom_rôle1 nom_rôle2 ...` : installe ou enlève un/des rôle(s)
- `ansible-galaxy role init nom_rôle` : crée un nouveau rôle
- `ansible-galaxy role info nom_rôle` : récupère les informations sur un rôle
- `ansible-galaxy collection install|remove nom_collection1 nom_collection2 ...` : installe ou enlève une/des collection(s)
- `ansible-galaxy rôle|collection -h` : affiche l'aide sur les rôles ou les collections

## Links
- More information: https://docs.ansible.com/projects/ansible/latest/cli/ansible-galaxy.html
