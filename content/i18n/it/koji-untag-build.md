# koji untag-build

Rimuove un tag da una o più build.

## Usage
- `koji untag-build tag NVR1 NVR2 ...` : rimuove un tag da una o più build
- `koji untag-build tag pkg1 pkg2 ... --all` : rimuovi il tag da tutte le versioni del pacchetto in questo tag
- `koji untag-build tag pkg1 pkg2 ... --non-latest` : rimuovi il tag da tutte le versioni del pacchetto in questo tag tranne l'ultima
- `koji untag-build tag NVR1 NVR2 ... -n` : modalità di prova
- `koji untag-build tag NVR1 NVR2 ... -v` : stampa dettagli
- `koji untag-build -h` : mostra l'aiuto

## Links
- More information: https://docs.pagure.org/koji/
