# pip list

Elenca i pacchetti Python installati.

## Usage
- `pip list` : elenca i pacchetti installati
- `pip list -o` : elenca i pacchetti obsoleti che possono essere aggiornati
- `pip list -u` : elenca i pacchetti aggiornati
- `pip list --format json` : elenca i pacchetti con formattazione JSON
- `pip list --not-required` : elenca i pacchetti che non sono richiesti da altri pacchetti
- `pip list --user` : elenca i pacchetti installati solo nel sito dell'utente (user-site)
- `pip list --exclude-editable` : elenca i pacchetti ed escludi i pacchetti modificabili (editable) dall'output
- `pip list --format freeze` : elenca i pacchetti nel formato freeze (a differenza di `pip freeze`, non mostra le informazioni sull'installazione modificabile)

## Links
- More information: https://pip.pypa.io/en/stable/cli/pip_list/
