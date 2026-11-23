# cryptsetup

Gestisce volumi crittografati `dm-crypt` e LUKS (Linux Unified Key Setup). Alcuni sottocomandi come `luksFormat` hanno la propria documentazione d'uso.

## Usage
- `cryptsetup luksFormat /dev/sdXY` : inizializza un volume LUKS con una passphrase (sovrascrive tutti i dati sulla partizione)
- `cryptsetup open /dev/sdXY nome_mapping` : apre un volume LUKS e crea un mapping decrittografato su `/dev/mapper/mapping_name`
- `cryptsetup status nome_mapping` : mostra informazioni su un mapping
- `cryptsetup close nome_mapping` : rimuove un mapping esistente
- `cryptsetup luksChangeKey /dev/sdXY` : cambia la passphrase di un volume LUKS
- `cryptsetup luksDump /dev/sdXY` : mostra le informazioni dell'intestazione LUKS e i metadati degli slot di chiave di un dispositivo crittografato

## Links
- More information: https://manned.org/cryptsetup
