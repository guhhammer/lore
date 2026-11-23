# pip inspect

Ispeziona l'ambiente Python e produci un rapporto in formato JSON.

## Usage
- `pip inspect` : ispeziona l'ambiente corrente
- `pip inspect >environment_report.json` : ispeziona e salva l'output in un file
- `pip inspect --local` : ispeziona solo i pacchetti installati localmente (non globali)
- `pip inspect --user` : ispeziona solo i pacchetti installati dall'utente
- `pip inspect --path percorso/dell/ambiente` : ispeziona i pacchetti in uno specifico percorso
- `pip inspect -v` : ispeziona con un output dettagliato (Nota: il flag `-v` può essere ripetuto per aumentare la verbosità)

## Links
- More information: https://pip.pypa.io/en/stable/cli/pip_inspect/
