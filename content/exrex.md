# exrex

Generate all/random matching strings for a `regex`. It can also simplify `regex`es.

## Usage
- `exrex 'regex'` : generate all possible strings that match a `regex`
- `exrex -r 'regex'` : generate a random string that matches a `regex`
- `exrex -m 100 'regex'` : generate at most 100 strings that match a `regex`
- `exrex -d ", " 'regex'` : generate all possible strings that match a `regex`, joined by a custom delimiter string
- `exrex -c 'regex'` : print count of all possible strings that match a `regex`
- `exrex -s 'ab|ac'` : simplify a `regex`
- `exrex '[oO0](_)[oO0]'` : print eyes
- `exrex '( {20}(\| *\\|-{22}|\|)|\.={50}| ( ){0,5}\\\.| {12}~{39})'` : print a boat

## Categories
text, documents

## Links
- More information: https://github.com/asciimoo/exrex#usage
- tldr page: https://tldr.inbrowser.app/pages/common/exrex
