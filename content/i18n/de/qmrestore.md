# qmrestore

Wiederherstellung von QemuServer vzdump Backups.

## Usage
- `qmrestore pfad/zu/vzdump-qemu-100.vma.lzo 100` : wiederherstellung einer virtuellen Maschine mittels Backupdatei auf dem ursprünglichen Speicher
- `qmrestore pfad/zu/vzdump-qemu-100.vma.lzo 100 --force true` : überschreibung einer bestehenden virtuellen Maschine auf dem ursprünglichen Speicher
- `qmrestore pfad/zu/vzdump-qemu-100.vma.lzo 100 --storage local` : wiederherstellung einer virtuellen Maschine auf einem bestimmten Speicher
- `qmrestore pfad/zu/vzdump-qemu-100.vma.lzo 100 --live-restore true` : sofortiger Start einer virtuellen Maschine bei gleichzeitiger Wiederherstellung im Hintergrund (nur bei Proxmox Backup Server)

## Links
- More information: https://pve.proxmox.com/pve-docs/qmrestore.1.html
