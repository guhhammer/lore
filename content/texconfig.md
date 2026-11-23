# texconfig

Configure TeX Live or teTeX. Superseded in many cases by `tlmgr`.

## Usage
- `texconfig conf` : display the current TeX configuration
- `texconfig paper a4|letter|...` : set the default paper size for supported programs
- `texconfig pdftex paper a4|letter|...` : set the default paper size for PDFTeX
- `texconfig init` : rebuild all TeX formats and font maps
- `texconfig init format` : rebuild a specific format
- `texconfig rehash` : refresh the TeX filename database

## Links
- More information: https://manned.org/texconfig
- tldr page: https://tldr.inbrowser.app/pages/common/texconfig
