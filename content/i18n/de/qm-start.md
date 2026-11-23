# qm start

Starte eine virtuelle Maschine per QEMU/KVM Virtual Machine Manager.

## Usage
- `qm start 100` : starte eine bestimmte virtuelle Maschine
- `qm start 100 --machine q35` : lege den QEMU Maschinentyp fest (etwa den zu emulierenden Prozessor)
- `qm start 100 --timeout 60` : starte eine bestimmte virtuelle Maschine mit einem Timeout (Zeitabschaltung) nach 60 Sekunden

## Links
- More information: https://pve.proxmox.com/pve-docs/qm.1.html#cli_qm_start
