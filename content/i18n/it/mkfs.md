# mkfs

Costruisce un filesystem Linux su una partizione del disco rigido. Questo comando è deprecato in favore degli strumenti specifici per filesystem: mkfs.tipo.

## Usage
- `sudo mkfs percorso/della/partizione` : costruisce un filesystem Linux ext2 su una partizione
- `sudo mkfs -t ext4 percorso/della/partizione` : costruisce un filesystem del tipo specificato
- `sudo mkfs -c -t ntfs percorso/della/partizione` : costruisce un filesystem del specificato e controlla la presenza di settori danneggiati

## Links
- More information: https://manned.org/mkfs
