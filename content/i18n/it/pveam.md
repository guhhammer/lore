# pveam

Gestisce i template dei container LXC.

## Usage
- `pveam u` : aggiorna il database dei template dei container
- `pveam a` : elenca i template disponibili
- `pveam d local nome_template` : scarica un template
- `pveam l local` : elenca i template scaricati
- `pveam a --section system|turnkeylinux|mail` : elenca i template disponibili in una sezione specifica
- `pveam r local:vztmpl/nome_template` : rimuovi un template

## Links
- More information: https://pve.proxmox.com/pve-docs/pveam.1.html
