# brew bundle

Bundler per Homebrew, Homebrew Cask e per il Mac App Store.

## Usage
- `brew bundle` : installa un pacchetto da un Brewfile nel percorso corrente
- `brew bundle --file percorso/del/file` : installa pacchetti da un Brewfile specifico in un percorso specifico
- `brew bundle dump` : crea un Brewfile con tutti i pacchetti installati
- `brew bundle cleanup --force` : disinstalla tutti i pacchetti non specificati nel Brewfile
- `brew bundle check` : controlla se c'è qualcosa da installare o da aggiornare nel Brewfile
- `brew bundle list --all` : mostra una lista di tutte le righe presenti nel Brewfile

## Links
- More information: https://docs.brew.sh/Manpage#bundle-subcommand
