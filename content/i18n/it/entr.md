# entr

Esegui comandi arbitrari al cambiamento di file.

## Usage
- `ag -l | entr make` : ricompila con `make` se qualsiasi file in quasiasi sottodirectory cambia
- `ls *.c | entr 'make && make test'` : ricompila e testa con `make` se qualsiasi file sorgente `.c` nella directory corrente cambia
- `ls *.rb | entr -r ruby main.rb` : invia il segnale `SIGTERM` ad un sottoprocesso ruby precedentemente avviato prima di eseguire `ruby main.rb`
- `ls *.sql | entr psql -f /_` : esegui un comando con il file cambiato (`/_`) come argomento

## Links
- More information: https://eradman.com/entrproject/
