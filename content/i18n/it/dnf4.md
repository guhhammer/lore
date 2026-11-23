# dnf4

Gestore di pacchetti per RHEL 8/9 e versioni precedenti di Fedora (pre-41).

## Usage
- `sudo dnf4 up` : aggiorna i pacchetti installati alle versioni più recenti disponibili
- `dnf4 se keyword1 keyword2 ...` : cerca pacchetti tramite parole chiave
- `dnf4 if pacchetto` : mostra i dettagli di un pacchetto
- `sudo dnf4 in pacchetto1 pacchetto2 ...` : installa un nuovo pacchetto (usa `--assumeyes` per confermare automaticamente tutte le richieste)
- `sudo dnf4 rm pacchetto1 pacchetto2 ...` : rimuove un pacchetto
- `dnf4 ls --installed` : elenca i pacchetti installati
- `dnf4 wp comando` : trova quali pacchetti forniscono un dato comando
- `dnf4 hist` : visualizza tutte le operazioni passate

## Links
- More information: https://wiki.archlinux.org/title/Pacman/Rosetta
