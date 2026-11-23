# dd

Converti e copia un file.

## Usage
- `dd if=percorso/del/file.iso of=/dev/disco_usb status=progress` : crea un disco USB avviabile da un file ISO e mostra il progresso
- `dd bs=4194304 conv=fsync if=/dev/disco_sorgente of=/dev/disco_destinazione` : clona un disco su un altro disco a blocchi con grandezza di 4 MiB e scarica le scritture prima che il comando termini
- `dd bs=100 count=1 if=/dev/urandom of=percorso/del/file_random` : genera un file con un numero specifico di byte randomici utilizzando il driver random del kernel
- `dd bs=1024 count=1000000 if=/dev/zero of=percorso/del/file_1GB` : testa la performance in scrittura di un disco
- `dd if=/dev/disco of=percorso/del/file.img status=progress` : crea un backup di sistema, salvalo in un file IMG (può essere ripristinato in seguito scambiando `if` e `of`), e mostra il progresso

## Links
- More information: https://manned.org/dd.1p
