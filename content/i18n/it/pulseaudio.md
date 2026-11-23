# pulseaudio

Programma che permette di gestire il daemon audio di sistema.

## Usage
- `pulseaudio --check` : controlla se PulseAudio è in esecuzione. Se il programma non è attivo viene restituito un exit code diverso da 0
- `pulseaudio --start` : esegue il daemon di PulseAudio in background
- `pulseaudio -k` : interrompe l'esecuzione del daemon di PulseAudio
- `pulseaudio --dump-modules` : mostra i moduli disponibili
- `pulseaudio -L "nome_modulo argomenti"` : carica un modulo all'interno del daemon in esecuzione con gli argomenti specificati

## Links
- More information: https://manned.org/pulseaudio
