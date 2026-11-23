# hledger add

Record new transactions with interactive prompting in the console.

## Usage
- `hledger add` : record new transactions, saving to the default journal file
- `hledger add -f path/to/2024.journal -f path/to/2023.journal` : add transactions to `2024.journal`, but also load `2023.journal` for completions
- `hledger add today 'best buy' expenses:supplies '$20'` : provide answers for the first four prompts
- `hledger add -h` : show `add`'s options and documentation with `$PAGER`
- `hledger help add` : show `add`'s documentation with `info` or `man` if available

## Categories
system, media, files

## Links
- More information: https://hledger.org/hledger.html#add
- tldr page: https://tldr.inbrowser.app/pages/common/hledger-add
