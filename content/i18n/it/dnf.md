# dnf

Gestore di pacchetti per Fedora 41+ e RHEL 10.

## Usage
- `sudo dnf up` : aggiorna i pacchetti installati alle versioni più recenti disponibili
- `dnf se keyword1 keyword2 ...` : cerca pacchetti tramite parole chiave
- `dnf if pacchetto` : mostra i dettagli di un pacchetto
- `sudo dnf in pacchetto1 pacchetto2 ...` : installa nuovi pacchetti (usa `--assumeyes` per confermare automaticamente tutte le richieste)
- `sudo dnf rm pacchetto1 pacchetto2 ...` : rimuove pacchetti
- `dnf ls --installed` : elenca i pacchetti installati
- `dnf provides comando` : trova quali pacchetti forniscono un dato comando
- `sudo dnf clean all|dbcache|expire-cache|metadata|packages` : pulisce i dati in cache

## Links
- More information: https://wiki.archlinux.org/title/Pacman/Rosetta
