# grub-mkrescue

Crea un'immagine avviabile GRUB per CD/USB/floppy.

## Usage
- `grub-mkrescue --output grub.iso .` : crea un ISO avviabile dalla directory corrente e lo salva come `grub.iso`
- `grub-mkrescue --directory /usr/lib/grub/i386-pc --output grub.iso path/to/source` : crea un ISO usando file GRUB da una directory personalizzata
- `grub-mkrescue --compress no|xz|gz|lzo --output grub.iso path/to/source` : usa la compressione per i file GRUB durante la creazione dell'immagine, impostando `no` disabilita la compressione
- `grub-mkrescue --disable-cli --output grub.iso path/to/source` : disabilita l'interfaccia a riga di comando GRUB nell'immagine generata
- `grub-mkrescue --modules "part_gpt iso9660" --output grub.iso path/to/source` : precarica moduli GRUB specifici nell'immagine
- `grub-mkrescue --output grub.iso -- -volid volume_name path/to/source` : passa opzioni aggiuntive direttamente a `xorriso`
- `grub-mkrescue -?` : mostra l'aiuto
- `grub-mkrescue --version` : mostra la versione
