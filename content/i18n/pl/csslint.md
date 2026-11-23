# csslint

Lintuj kod CSS.

## Usage
- `csslint plik.css` : lintuj pojedyńczy plik CSS
- `csslint plik1.css plik2.css ...` : lintuj wiele plików CSS
- `csslint --list-rules` : wymień wszystkie możliwe reguły stylu
- `csslint --errors=errors,universal-selector,imports plik.css` : określ pewne reguły jako błędy (które powodują niezerowy kod wyjścia)
- `csslint --warnings=box-sizing,selector-max,floats plik.css` : określ pewne reguły jako ostrzeżenia
- `csslint --ignore=ids,rules-count,shorthand plik.css` : ignoruj określone reguły

## Links
- More information: https://github.com/CSSLint/csslint/wiki/Command-line-interface
