# erl

Esegui e gestisci programmi nel linguaggio di programmazione Erlang.

## Usage
- `erlc file && erl -noshell 'modulo:funzione(argomenti), init:stop().'` : compila ed esegui un programma Erlang sequenziale come un comune script e poi esci
- `erl -remsh nome_nodo@hostname -sname soprannome -hidden -setcookie cookie_nodo_remoto` : connetti ad un nodo Erlang in esecuzione
- `erl -pa directory_con_file_beam` : fai caricare alla shell Erlang dei moduli da una directory

## Links
- More information: https://erlang.org/documentation/doc-16.0/erts-16.0/doc/html/erl_cmd.html
