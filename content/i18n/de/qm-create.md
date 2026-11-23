# qm create

Erzeugung einer virtuellen Maschine per QEMU/KVM Virtual Machine Manager.

## Usage
- `qm create 100` : erzeuge eine virtuelle Maschine
- `qm create 100 --start 1` : erzeuge eine virtuelle Maschine und starte sie unmittelbar danach im Anschluss
- `qm create 100 --ostype win10` : lege den Typ des Betriebssystems auf der virtuellen Maschine fest
- `qm create 100 --archive pfad/zu/backup_file.tar --force 1` : ersetze eine bestehende virtuelle Maschine (setzt deren Archivierung voraus)
- `qm create 100 --hookscript pfad/zu/skript.pl` : lege ein Skript fest, welches automatisch abhängig vom Zustand der virtuellen Maschine ausgelöst werden soll

## Links
- More information: https://pve.proxmox.com/pve-docs/qm.1.html#cli_qm_create
