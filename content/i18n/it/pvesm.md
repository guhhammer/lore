# pvesm

Gestisce lo storage di Proxmox.

## Usage
- `pvesm st` : mostra lo stato di tutti i datastore
- `pvesm l nome_storage` : elenca i contenuti dello storage
- `pvesm pa nome_storage:tipo_contenuto/nome_file` : mostra il percorso di un file archiviato
- `pvesm ad dir nome_storage --path percorso/della/directory` : aggiunge uno storage di tipo directory
- `pvesm se nome_storage --content iso,images,backup,vztmpl,rootdir,snippets,import` : configura uno storage per contenere tipi di contenuto specifici
- `pvesm f local:iso/archlinux-2025.08.01-x86_64.iso` : elimina un file dallo storage
- `pvesm r nome_storage` : rimuove uno storage

## Links
- More information: https://pve.proxmox.com/pve-docs/pvesm.1.html
