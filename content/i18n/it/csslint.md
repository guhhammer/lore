# csslint

Un linter per codice CSS.

## Usage
- `csslint file.css` : esegui il linting di un singolo file CSS
- `csslint file1.css file2.css ...` : esegui il linting di file CSS multipli
- `csslint --list-rules` : elenca tutte le possibili regole di stile
- `csslint --errors=errors,universal-selector,imports file.css` : specifica certe regole come errori (che risulteranno in un codice d'uscita diverso da zero)
- `csslint --warnings=box-sizing,selector-max,floats file.css` : specifica certe regole come warning
- `csslint --ignore=ids,rules-count,shorthand file.css` : specifica certe regole da essere completamente ignorate

## Links
- More information: https://github.com/CSSLint/csslint/wiki/Command-line-interface
