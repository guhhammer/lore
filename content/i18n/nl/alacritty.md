# alacritty

Cross-platform, GPU-versnelde terminalemulator.

## Usage
- `alacritty` : start een nieuw Alacritty-proces en maak een venster aan
- `alacritty --daemon` : start de Alacritty-daemon (zonder een venster te maken)
- `alacritty msg create-window` : maak een nieuw venster met behulp van het reeds lopende Alacritty-proces
- `alacritty --working-directory pad/naar/map` : start de shell in een specifieke map (werkt ook met `alacritty msg create-window`)
- `alacritty -e commando` : voer een commando uit in een nieuw Alacritty-venster (werkt ook met `alacritty msg create-window`)
- `alacritty --config-file pad/naar/config.toml` : geef een alternatief configuratiebestand op (standaard ingesteld op `$XDG_CONFIG_HOME/alacritty/alacritty.toml`)

## Links
- More information: https://manned.org/alacritty
