# cryptsetup open

Crea un mapping decrittografato di un volume crittografato. Nota: Con TRIM abilitato, si può verificare una minima perdita di dati sotto forma di informazioni sui blocchi liberati, forse sufficienti per determinare il filesystem in uso. Tuttavia, è consigliabile abilitarlo comunque, perché i dati al suo interno sono ancora al sicuro e gli SSD senza TRIM si usurano più velocemente.

## Usage
- `cryptsetup open /dev/sdXY nome_mapping` : apre un volume LUKS e crea un mapping decrittografato su `/dev/mapper/mapping_name`
- `cryptsetup open -k path/to/file /dev/sdXY nome_mapping` : utilizza un keyfile invece di una passphrase
- `cryptsetup open --allow-discards /dev/sdXY nome_mapping` : permette l'uso di TRIM sul dispositivo
- `cryptsetup open --allow-discards --persistent /dev/sdXY nome_mapping` : scrive l'opzione `--allow-discards` nell'intestazione LUKS (l'opzione verrà quindi sempre utilizzata quando si apre il dispositivo)
- `cryptsetup open -r /dev/sdXY nome_mapping` : apre un volume LUKS e rende il mapping decrittografato di sola lettura

## Links
- More information: https://manned.org/cryptsetup-open
