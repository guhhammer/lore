# aura

Package manager Aura: gestore pacchetti sicuro e multilingua per Arch Linux e AUR.

## Usage
- `aura -As keyword|regex` : cerca pacchetti dall'AUR
- `aura -A pacchetto` : installa un pacchetto dall'AUR
- `aura -Akua` : aggiorna tutti i pacchetti AUR in modalità verbose e rimuove tutte le dipendenze di compilazione
- `aura -S pacchetto` : installa un pacchetto dai repository ufficiali
- `aura -Syu` : sincronizza e aggiorna tutti i pacchetti dai repository ufficiali
- `aura -Rsu pacchetto` : rimuove un pacchetto e le sue dipendenze
- `aura -Oj` : rimuove i pacchetti orfani (installati come dipendenze ma non più richiesti)

## Links
- More information: https://github.com/fosskers/aura
