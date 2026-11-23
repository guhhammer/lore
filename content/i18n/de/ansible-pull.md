# ansible-pull

Laden eines Ansible-Playbooks aus einem VCS-Repository und ausführen auf dem lokalen Host.

## Usage
- `ansible-pull -U repository_url` : laden eines Playbooks aus einem VCS und ausführen des standardmässigen `local.yml` Playbooks
- `ansible-pull -U repository_url playbook` : laden eines Playbooks aus einem VCS und ausführen eines spezifischen Playbooks
- `ansible-pull -U repository_url -C branch playbook` : laden eines Playbooks aus einem VCS unter angabe eines bestimmten branches und ausführen eines spezifischen Playbooks
- `ansible-pull -U repository_url -i hosts_datei playbook` : laden eines Playbooks aus einem VCS und ausführen eines spezifischen Playbooks unter angabe einer Hosts-Datei

## Links
- More information: https://docs.ansible.com/projects/ansible/latest/cli/ansible-pull.html
