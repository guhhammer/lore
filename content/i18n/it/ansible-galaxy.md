# ansible-galaxy

Esegui varie operazioni inerenti ai Ruoli e alle Collezioni in Ansible.

## Usage
- `ansible-galaxy ruolo|collezione list` : lista i ruoli o le collezioni installate
- `ansible-galaxy role search keyword -vvvvvv` : cerca un ruolo con vari livelli di verbosità (`-v` deve essere specificato alla fine)
- `ansible-galaxy role install|remove nome_ruolo1 nome_ruolo2 ...` : installa o rimuovi ruoli
- `ansible-galaxy role init nome_ruolo` : crea un nuovo ruolo
- `ansible-galaxy role info nome_ruolo` : ottieni informazioni inerenti a un ruolo
- `ansible-galaxy collection install|remove nome_collezione1 nome_collezione2 ...` : installa o rimuovi collezioni
- `ansible-galaxy ruolo|collezione -h` : mostra aiuto su ruoli o collezioni

## Links
- More information: https://docs.ansible.com/projects/ansible/latest/cli/ansible-galaxy.html
