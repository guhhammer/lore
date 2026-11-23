# alacritty

Emulatore di terminale multipiattaforma con accelerazione GPU.

## Usage
- `alacritty` : avvia un nuovo processo Alacritty e crea una finestra
- `alacritty --daemon` : avvia il demone Alacritty (senza creare una finestra)
- `alacritty msg create-window` : crea una nuova finestra usando il processo Alacritty già in esecuzione
- `alacritty --working-directory percorso/della/directory` : avvia la shell in una directory specifica (funziona anche con `alacritty msg create-window`)
- `alacritty -e comando` : esegui un comando in una nuova finestra Alacritty (funziona anche con `alacritty msg create-window`)
- `alacritty --config-file percorso/al/config.toml` : usa un file di configurazione alternativo (predefinito: `$XDG_CONFIG_HOME/alacritty/alacritty.toml`)

## Links
- More information: https://manned.org/alacritty
