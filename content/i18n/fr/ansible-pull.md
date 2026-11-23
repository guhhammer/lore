# ansible-pull

Récupère les playbook ansible depuis un dépôt VCS et exécute les en local.

## Usage
- `ansible-pull -U url_du_dépôt` : récupère le playbook depuis un VCS et exécute le fichier par défaut `local.yml` du playbook
- `ansible-pull -U url_du_dépôt playbook` : récupère le playbook depuis un VCS et exécute un playbook spécifique
- `ansible-pull -U url_du_dépôt -C branche playbook` : récupère un playbook depuis un VCS sur une branche spécifique et exécute ce dernier
- `ansible-pull -U url_du_dépôt -i fichier_hôtes playbook` : récupère un playbook depuis un VCS, spécifie les fichiers hôtes et exécute un playbook spécifique

## Links
- More information: https://docs.ansible.com/projects/ansible/latest/cli/ansible-pull.html
