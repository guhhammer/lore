# conda init

Inizializza conda per l'interazione con la shell. La maggior parte delle shell deve essere chiusa e riaperta affinché le modifiche abbiano effetto.

## Usage
- `conda init zsh|bash|powershell|fish|tcsh|xonsh` : inizializza una shell specifica (se non specificata, predefinita `bash` su UNIX e `powershell` su Windows)
- `conda init --all` : inizializza tutte le shell disponibili
- `conda init --system` : inizializza conda per tutti gli utenti del sistema
- `conda init --no-user` : non inizializzare conda per l'utente corrente
- `conda init --condabin` : aggiungi la directory `condabin/` a `$PATH`
- `conda init --reverse` : annulla gli effetti dell'ultimo comando `conda init`

## Links
- More information: https://docs.conda.io/projects/conda/en/latest/commands/init.html
