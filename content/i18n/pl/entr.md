# entr

Uruchom dowolną komendę, gdy zmieni się plik.

## Usage
- `ag --files-with-matches | entr make` : przebuduj projekt używając `make`, jeżeli zmiemi się którykolwiek z plików w podkatalogu
- `ls *.c | entr 'make && make test'` : jeżeli zmieni się którykowliek z plików źródłowych `.c` w obecnym katalogu, przebuduj i uruchom testy używając `make`
- `ls *.rb | entr -r ruby main.rb` : wyślij `SIGTERM` do wszystkich uruchomionych poprzednio podprocesów ruby przed wykonaniem `ruby main.rb`
- `ls *.sql | entr psql -f /_` : uruchom komendę przekazując zmieniony plik (`/_`) jako jej argument

## Links
- More information: https://eradman.com/entrproject/
