# aa-update-browser

Aggiorna i profili browser di AppArmor per utilizzare le astrazioni supportate. Parte della suite AppArmor.

## Usage
- `sudo aa-update-browser -l` : lista i profili di astrazione browser disponibili
- `sudo aa-update-browser -d path/to/profile` : mostra quali modifiche verrebbero apportate a un profilo senza applicarle (simulazione)
- `sudo aa-update-browser -u astrazione1,astrazione2,... path/to/profile` : aggiorna un profilo con astrazioni specifiche
- `sudo aa-update-browser -u '' path/to/profile` : rimuove tutte le astrazioni da un profilo
- `aa-update-browser -h` : mostra l'aiuto

## Links
- More information: https://manned.org/aa-update-browser
