# btrfs device

Gestisce i dispositivi in un filesystem btrfs.

## Usage
- `sudo btrfs d a percorso/al/block_device1 percorso/al/block_device2 ... percorso/al/filesystem_btrfs` : aggiunge uno o più dispositivi a un filesystem btrfs
- `sudo btrfs d rem percorso/al/device1|device_id1 percorso/al/device2|device_id2 ...` : rimuove un dispositivo da un filesystem btrfs
- `sudo btrfs d st percorso/al/filesystem_btrfs` : mostra le statistiche degli errori
- `sudo btrfs d sc -d` : scansiona tutti i dischi e informa il kernel di tutti i filesystem btrfs rilevati
- `sudo btrfs d u percorso/al/filesystem_btrfs` : mostra statistiche dettagliate di allocazione per disco

## Links
- More information: https://btrfs.readthedocs.io/en/latest/btrfs-device.html
