# ansible

Gestisce gruppi di computer da remoto via SSH. Usa il file `/etc/ansible/hosts` per aggiungere nuovi gruppi/host. Alcuni comandi aggiuntivi, come `galaxy`, hanno la propria documentazione.

## Usage
- `ansible gruppo --list-hosts` : elenca gli host appartenenti ad un gruppo
- `ansible gruppo -m ping` : invia un ping ad un gruppo di host invocando il modulo "ping"
- `ansible gruppo -m setup` : mostra informazioni su un gruppo di host invocando il modulo "setup"
- `ansible gruppo -m command -a 'comando_da_eseguire'` : esegui un comando su un gruppo di host invocando il modulo "command" con degli argomenti
- `ansible gruppo -b --ask-become-pass -m command -a 'comando'` : esegui un comando con privilegi di amministratore
- `ansible gruppo -i file_inventory -m command -a 'comando'` : esegui un comando usando un file di inventory personalizzato
- `ansible localhost -m debug -a 'var=groups.keys()'` : elenca i gruppi in un inventory

## Links
- More information: https://docs.ansible.com/projects/ansible/latest/cli/ansible.html
