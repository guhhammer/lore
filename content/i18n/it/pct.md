# pct

Gestisce container LXC in Proxmox.

## Usage
- `pct list` : elenca tutti i container
- `pct start|stop|reboot 100` : avvia/Ferma/Riavvia un container specifico
- `pct en 100` : accedi alla shell di un container specifico
- `pct cr 100 local:vztmpl/distro-name.tar.zst --rootfs local-lvm:4` : crea un container da un template con dimensione 4GB
- `pct resi 100 rootfs|mpX 20G` : ridimensiona il disco del container a 20G
- `pct conf 100` : mostra la configurazione di un container, specificando il suo ID
- `pct sn 100 my-snapshot --description Descrizione dello snapshot` : crea uno snapshot di un container specifico con descrizione
- `pct des 100 --purge` : distrugge un container e rimuove tutte le risorse correlate

## Links
- More information: https://pve.proxmox.com/pve-docs/pct.1.html
