# ansible

Ansible configures machines over ssh from playbooks, with nothing to install on them. This page covers inventories, one-off commands, playbooks, vault and roles.

## Tasks
### Inventory and reach
- `ansible all -i inventory.ini -m ping` : check every host answers
- `ansible-inventory -i inventory.ini --list` : show the inventory as Ansible reads it
- `ansible all -i HOST, -m ping` : try one host without an inventory file (note the comma)
- `ansible web -i inventory.ini -m setup` : print the facts of a group
### One-off commands
- `ansible all -i inventory.ini -a "uptime"` : run a command on all hosts
- `ansible web -i inventory.ini -b -m apt -a "name=nginx state=present"` : install a package as root (Debian, Ubuntu)
- `ansible web -i inventory.ini -b -m service -a "name=nginx state=restarted"` : restart a service
- `ansible web -i inventory.ini -m copy -a "src=FILE dest=/tmp/FILE"` : copy a file
- `ansible all -i inventory.ini -u USER --ask-pass -m ping` : log in with a password
### Playbooks
- `ansible-playbook -i inventory.ini site.yml` : run a playbook
- `ansible-playbook -i inventory.ini site.yml --check --diff` : show what would change, change nothing
- `ansible-playbook -i inventory.ini site.yml --limit HOST` : only one host or group
- `ansible-playbook -i inventory.ini site.yml --tags NAME` : only tasks with a tag
- `ansible-playbook -i inventory.ini site.yml -K` : ask for the sudo password
- `ansible-playbook site.yml --syntax-check` : find syntax errors
- `ansible-playbook -i inventory.ini site.yml -e "var=value"` : pass a variable
- `ansible-playbook -i inventory.ini site.yml -vvv` : more detail when something fails
### Secrets (vault)
- `ansible-vault create secrets.yml` : make an encrypted file
- `ansible-vault edit secrets.yml` : edit it
- `ansible-vault encrypt vars.yml` : encrypt an existing file
- `ansible-playbook site.yml --ask-vault-pass` : run with the vault password
### Roles and collections
- `ansible-galaxy init ROLE` : a role skeleton
- `ansible-galaxy install -r requirements.yml` : install roles and collections a project needs
- `ansible-galaxy collection install community.general` : install a collection
- `ansible-doc -l` : list every module
- `ansible-doc ansible.builtin.copy` : the manual of one module
- `ansible-lint site.yml` : check a playbook for bad practice

## Notes
- Playbooks should be idempotent: running them twice changes nothing the second time.
- `--check` cannot predict every task; read its result, then run for real.
- `-b` (become) is sudo; `-K` asks for its password.
- YAML cares about spaces: indent with spaces, never tabs.
- Install it from your package manager or pipx as the official page describes.

## See also
- terraform
- ssh
- docker

## Links
- Docs: https://docs.ansible.com/ansible/latest/
- Install: https://docs.ansible.com/ansible/latest/installation_guide/

## Categories
development system
