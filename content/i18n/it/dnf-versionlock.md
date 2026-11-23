# dnf versionlock

Protegge i pacchetti da aggiornamenti a versioni più recenti. Non predefinito in `dnf` ma supportato tramite `dnf-plugins-core`.

## Usage
- `dnf versionlock` : elenca le voci correnti di versionlock
- `dnf versionlock add pacchetto` : aggiunge un versionlock per tutti i pacchetti disponibili che corrispondono alla specifica
- `dnf versionlock exclude pacchetto` : aggiunge un'esclusione (nel versionlock) per i pacchetti disponibili che corrispondono alla specifica
- `dnf versionlock delete pacchetto` : rimuove le voci corrispondenti di versionlock
- `dnf versionlock clear` : rimuove tutte le voci di versionlock

## Links
- More information: https://dnf-plugins-core.readthedocs.io/en/latest/versionlock.html
