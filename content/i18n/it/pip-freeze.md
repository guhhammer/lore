# pip freeze

Elenca i pacchetti installati nel formato dei requisiti (requirements).

## Usage
- `pip freeze` : elenca i pacchetti installati
- `pip freeze >requirements.txt` : scrivi i pacchetti installati nel file `requirements.txt`
- `pip freeze -l` : elenca i pacchetti installati in un ambiente virtuale, escludendo i pacchetti installati globalmente
- `pip freeze --user` : elenca i pacchetti installati nel sito dell'utente (user-site)
- `pip freeze --all` : elenca tutti i pacchetti, inclusi `pip`, `distribute`, `setuptools` e `wheel` (vengono saltati per impostazione predefinita)

## Links
- More information: https://pip.pypa.io/en/stable/cli/pip_freeze/
