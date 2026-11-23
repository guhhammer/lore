# pip wheel

Crea archivi wheel per pacchetti e dipendenze.

## Usage
- `pip wheel pacchetto` : crea un file wheel per un pacchetto
- `pip wheel -r percorso/del/requirements.txt` : crea file wheel per i pacchetti in un file dei requisiti (requirements)
- `pip wheel pacchetto -w percorso/della/directory` : crea un file wheel in una directory specifica
- `pip wheel pacchetto --no-deps` : crea un file wheel senza dipendenze
- `pip wheel percorso/del/progetto` : crea un file wheel da un progetto locale
- `pip wheel git+https://github.com/nome_utente/repository.git` : crea un file wheel da un repository Git

## Links
- More information: https://pip.pypa.io/en/stable/cli/pip_wheel/
