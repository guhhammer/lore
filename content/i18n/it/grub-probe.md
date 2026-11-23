# grub-probe

Analizza le informazioni del dispositivo per un percorso o dispositivo particolare.

## Usage
- `sudo grub-probe -t fs /boot/grub` : ottiene il modulo filesystem GRUB per un percorso
- `sudo grub-probe -t device /boot/grub` : ottiene il dispositivo di sistema che contiene un percorso
- `sudo grub-probe -t drive /dev/sdX -d` : ottiene il nome del disco GRUB per un dispositivo di sistema
- `sudo grub-probe -t fs_uuid /boot/grub` : ottiene l'UUID del filesystem
- `sudo grub-probe -t fs_label /boot/grub` : ottiene l'etichetta del filesystem
- `sudo grub-probe -t msdos_parttype /dev/sdX` : ottiene il codice del tipo di partizione MBR (due cifre esadecimali)
- `sudo grub-probe -t drive /boot/grub -m path/to/custom_device.map` : analizza usando una mappa dispositivi personalizzata
