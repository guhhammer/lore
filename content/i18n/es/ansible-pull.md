# ansible-pull

Extrae playbooks ansible de un repositorio VCS y los ejecuta para el host local.

## Usage
- `ansible-pull -U url_repositorio` : extrae un playbook de un VCS y ejecuta `local.yml` del playbook por defecto
- `ansible-pull -U url_repositorio playbook` : extrae un playbook de un VCS y ejecuta un playbook específico
- `ansible-pull -U url_repositorio -C rama playbook` : extrae un playbook de un VCS en una rama determinada y ejecuta un playbook específico
- `ansible-pull -U url_repositorio -i archivo_hosts playbook` : extrae un playbook de un VCS, en tanto especificando un archivo hosts y ejecuta un playbook específico

## Links
- More information: https://docs.ansible.com/projects/ansible/latest/cli/ansible-pull.html
