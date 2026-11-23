# badblocks

Cerca blocchi danneggiati su un dispositivo. Alcuni utilizzi di `badblocks` possono causare azioni distruttive, come la cancellazione di tutti i dati su un disco, inclusa la tabella delle partizioni.

## Usage
- `sudo badblocks /dev/sdX` : cerca blocchi danneggiati su un disco con test di sola lettura non distruttivo
- `sudo badblocks -n /dev/sdX` : cerca blocchi danneggiati su un disco non montato con test di lettura-scrittura non distruttivo
- `sudo badblocks -w /dev/sdX` : cerca blocchi danneggiati su un disco non montato con test di scrittura distruttivo
- `sudo badblocks -svw /dev/sdX` : usa il test di scrittura distruttivo e mostra il progresso verboso
- `sudo badblocks -o percorso/del/file -w /dev/sdX` : in modalità distruttiva, salva i blocchi trovati in un file
- `sudo badblocks -w -b 4096 -c 65536 /dev/sdX` : usa modalità distruttiva con velocità migliorata: dimensione blocco 4K e count blocchi 64K

## Links
- More information: https://manned.org/badblocks
