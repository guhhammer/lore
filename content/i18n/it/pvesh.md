# pvesh

Interfaccia con l'API di Proxmox VE.

## Usage
- `pvesh g /nodes` : elenca i nodi disponibili
- `pvesh g /nodes/nome_nodo/lxc|qemu` : mostra informazioni dettagliate su container o macchine virtuali
- `pvesh l /` : scopri i percorsi API
- `pvesh u /pools` : mostra le istruzioni di utilizzo del percorso API
- `pvesh create /storage --storage id_archivio --vgname volume_group --type lvmthin --thinpool nome_thinpool --content tipo_contenuto1,tipo_contenuto2,...` : aggiungi un nuovo thinpool a Proxmox

## Links
- More information: https://pve.proxmox.com/pve-docs/pvesh.1.html
