# dnf clean

Esegue la pulizia dei file temporanei mantenuti per i repository basati su Red Hat.

## Usage
- `dnf clean dbcache` : rimuove i file cache generati dai metadati del repository
- `dnf clean expire-cache` : marca i metadati del repository come scaduti
- `dnf clean metadata` : rimuove i metadati del repository
- `dnf clean packages` : rimuove qualsiasi pacchetto in cache dal sistema
- `dnf clean all` : pulisce tutti i metadati specifici del repository DNF e i file in cache (tutti i precedenti)

## Links
- More information: https://dnf.readthedocs.io/en/latest/command_ref.html#clean-command
