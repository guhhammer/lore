# ansible-pull

Scarica playbook Ansible da un repository VCS ed eseguili per l'host locale.

## Usage
- `ansible-pull -U url_repository` : scarica un playbook da VCS ed esegui il playbook `local.yml` predefinito
- `ansible-pull -U url_repository playbook` : scarica un playbook da VCS ed esegui un playbook specifico
- `ansible-pull -U url_repository -C branch playbook` : scarica un playbook da VCS da un branch specifico ed esegui un playbook specifico
- `ansible-pull -U url_repository -i file_host playbook` : scarica un playbook da VCS, specifica file host ed esegui un playbook specifico

## Links
- More information: https://docs.ansible.com/projects/ansible/latest/cli/ansible-pull.html
