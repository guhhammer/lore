# ansible-doc

Visualizza informazioni sui moduli installati nelle librerie Ansible. Visualizza un elenco conciso dei plugin e delle loro brevi descrizioni.

## Usage
- `ansible-doc -l` : elenca i plugin di azione disponibili (moduli)
- `ansible-doc -t become|cache|callback|cliconf|connection|... -l` : elenca i plugin disponibili di un tipo specifico
- `ansible-doc plugin_name` : mostra informazioni su uno specifico plugin di azione (modulo)
- `ansible-doc -t become|cache|callback|cliconf|connection|... plugin_name` : mostra informazioni su un plugin con un tipo specifico
- `ansible-doc -s plugin_name` : mostra lo snippet del playbook per il plugin di azione (moduli)
- `ansible-doc -j plugin_name` : mostra informazioni su un plugin di azione (modulo) come JSON

## Links
- More information: https://docs.ansible.com/projects/ansible/latest/cli/ansible-doc.html
