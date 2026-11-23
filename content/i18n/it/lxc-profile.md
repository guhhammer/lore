# lxc profile

Gestisce i profili per i container LXD.

## Usage
- `lxc profile list` : elenca tutti i profili disponibili
- `lxc profile show nome_profilo` : mostra la configurazione di un profilo specifico
- `lxc profile edit nome_profilo` : modifica un profilo specifico nell'editor predefinito
- `lxc <config.yaml profile edit nome_profilo` : modifica un profilo specifico importando i valori di configurazione da un file
- `lxc launch container_image nome_container -p profilo1 -p profilo2` : avvia un nuovo container con profili specifici
- `lxc profile assign nome_container profilo1,profilo2` : cambia i profili di un container in esecuzione

## Links
- More information: https://canonical.com/lxd/docs/latest/reference/manpages/lxc/profile/
