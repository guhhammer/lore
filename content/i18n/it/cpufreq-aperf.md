# cpufreq-aperf

Calcola la frequenza media della CPU in un periodo di tempo.

## Usage
- `sudo cpufreq-aperf` : avvia il calcolo, impostando come predefinito tutti i core della CPU e un intervallo di aggiornamento di 1 secondo
- `sudo cpufreq-aperf -c 1` : avvia il calcolo solo per la CPU 1
- `sudo cpufreq-aperf -i 3` : avvia il calcolo con un intervallo di aggiornamento di 3 secondi per tutti i core della CPU
- `sudo cpufreq-aperf -o` : calcola solo una volta

## Links
- More information: https://manned.org/cpufreq-aperf
