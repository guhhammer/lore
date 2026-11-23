# ansible-lint

Applica regole e segui le best practice con i tuoi contenuti di automazione.

## Usage
- `ansible-lint percorso/del/file_playbook percorso/della/role_directory` : lint un playbook specifico e una directory di ruoli
- `ansible-lint -x regola1,regola2,... percorso/del/file_playbook` : lint un playbook escludendo regole specifiche
- `ansible-lint -o -p percorso/del/file_playbook` : lint un playbook in modalità offline e formatta l'output come PEP8
- `ansible-lint -r percorso/della/custom_rules_directory percorso/del/file_playbook` : lint un playbook utilizzando una directory di regole personalizzate
- `ansible-lint percorso/della/project_directory` : lint ricorsivamente tutto il contenuto Ansible in una directory specificata

## Links
- More information: https://docs.ansible.com/projects/lint/
