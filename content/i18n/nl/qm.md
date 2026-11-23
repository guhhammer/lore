# qm

QEMU/KVM Virtual Machine Manager. Sommige subcommando's zoals `list`, `start`, `stop`, `clone`, etc. hebben hun eigen documentatie.

## Usage
- `qm list` : toon alle virtuele machines
- `qm cr 100 --scsi0 local-lvm:4 --net0 e1000 --cdrom local:iso/proxmox-mailgateway_2.1.iso` : maak een virtuele machine aan met een 4 GB SCSI-schijf op de `local-lvm`-opslag en een ID van 100, gebruikmakend van een ISO-bestand dat op de `local`-opslag is geüpload
- `qm co 100` : toon de configuratie van een virtuele machine, met vermelding van de ID
- `qm start 100` : start een specifieke virtuele machine
- `qm shu 100 && qm w 100` : stuur een verzoek tot afsluiten en wacht vervolgens tot de virtuele machine gestopt is
- `qm des 100 --purge` : verwijder een virtuele machine en al zijn gerelateerde bronnen

## Links
- More information: https://pve.proxmox.com/pve-docs/qm.1.html
