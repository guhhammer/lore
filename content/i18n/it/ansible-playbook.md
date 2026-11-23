# ansible-playbook

Esegui task definiti nel playbook su macchine remote via SSH.

## Usage
- `ansible-playbook playbook` : esegui task nel playbook
- `ansible-playbook playbook -i file_inventory` : esegui task nel playbook con inventory host personalizzato
- `ansible-playbook playbook -e "variabile1=valore1 variabile2=valore2"` : esegui task nel playbook con variabili extra definite da linea di comando
- `ansible-playbook playbook -e "@variabili.json"` : esegui task nel playbook con variabili extra definite in un file JSON
- `ansible-playbook playbook -t tag1,tag2` : esegui task nel playbook per i tag specificati
- `ansible-playbook playbook --start-at-task nome_task` : esegui task nel playbook partendo da un task specifico
- `ansible-playbook playbook -C -D` : esegui task nel playbook senza apportare modifiche (dry-run)

## Links
- More information: https://docs.ansible.com/projects/ansible/latest/cli/ansible-playbook.html
