# tuc

Corta texto (o bytes) donde coincida un delimitador, luego conserva las partes deseadas. Una versión más fácil de usar y potente de `cut` con valores por defecto sensibles.

## Usage
- `echo "foo bar baz" | tuc -d ' ' -f 3,2,1` : corta y reorganiza campos
- `echo "foo bar baz" | tuc -d ' ' -r ' ➡ '` : sustituye el delimitador `space` por una flecha
- `echo "foo bar    baz" | tuc -d ' ' -f 2:` : mantiene un rango de campos
- `echo "a,b, c" | tuc -e '[, ]+' -f 1,3` : corta usando expresiones regulares
- `echo "foo bar baz" | tuc -d ' ' --json` : genera salida JSON

## Links
- More information: https://github.com/riquito/tuc#help
