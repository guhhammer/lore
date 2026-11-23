# pip lock

Blocca i pacchetti Python e le loro dipendenze in un file riproducibile. Funzionalità sperimentale di `pip`.

## Usage
- `pip lock -e .` : genera un file `pylock.toml` per il progetto corrente
- `pip lock -r percorso/del/requirements.txt` : blocca le dipendenze da un file dei requisiti (requirements)
- `pip lock -o percorso/del/lockfile.toml` : specifica un file di output personalizzato per il blocco
- `pip lock pacchetto` : blocca uno specifico pacchetto e le sue dipendenze

## Links
- More information: https://pip.pypa.io/en/stable/cli/pip_lock/
