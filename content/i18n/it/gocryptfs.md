# gocryptfs

Filesystem crittografato scritto in Go.

## Usage
- `gocryptfs -init percorso/della/directory_cifrata` : inizializzare un filesystem crittografato
- `gocryptfs percorso/della/directory_cifrata percorso/del/punto_di_mount` : montare un filesystem crittografato
- `gocryptfs --masterkey percorso/della/directory_cifrata percorso/del/punto_di_mount` : montare un filesystem usando la master key invece della password
- `gocryptfs --passwd percorso/della/directory_cifrata` : cambiare la password
- `gocryptfs --reverse percorso/della/directory percorso/della/directory_cifrata` : generare uno snapshot cifrato di una directory

## Links
- More information: https://github.com/rfjakob/gocryptfs#use
