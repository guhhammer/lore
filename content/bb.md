# bb

Native Clojure interpreter for scripting.

## Usage
- `bb -e "(+ 1 2 3)"` : evaluate an expression
- `bb -f path/to/script.clj` : evaluate a script file
- `printf "first\nsecond" | bb -i "(map clojure.string/capitalize *input*)"` : bind input to a sequence of lines from `stdin`
- `echo "{:key 'val}" | bb -I "(:key (first *input*))"` : bind Input to a sequence of EDN (Extensible Data Notation) values from `stdin`

## Categories
development

## Links
- More information: https://book.babashka.org/#usage
- tldr page: https://tldr.inbrowser.app/pages/common/bb
