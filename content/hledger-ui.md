# hledger-ui

A terminal interface (TUI) for `hledger`, a robust, friendly plain text accounting app.

## Usage
- `hledger-ui` : start in the main menu screen, reading from the default journal file
- `hledger-ui --theme terminal|greenterm|dark` : start with a different color theme
- `hledger-ui --bs -t -3` : start in the balance sheet accounts screen, showing hierarchy down to level 3
- `hledger-ui --register assets:bank:checking -C -w` : start in this account's screen, showing cleared transactions, and reload on change
- `hledger-ui -f path/to/2024.journal -f path/to/2024-prices.journal --value now` : read two journal files, and show amounts as current value when known
- `hledger-ui --info` : show the manual in Info format, if possible
- `hledger-ui -h` : display help

## Categories
network, desktop, shell

## Links
- More information: https://hledger.org/hledger-ui.html
- tldr page: https://tldr.inbrowser.app/pages/common/hledger-ui
