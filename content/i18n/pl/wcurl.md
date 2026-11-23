# wcurl

Prosty wrapper na narzędzie `curl` do łatwego pobierania plików.

## Usage
- `wcurl https://example.com/index.html` : pobierz zawartość URL do pliku wskazanego przez ten URL (w tym wypadku `index.html`)
- `wcurl -o ścieżka/do/pliku https://example.com/index.html` : pobierz zawartość URL do pliku o określonej nazwie
- `wcurl --curl-options "--progress-bar --http2" https://example.com/index.html` : pobierz zawartość URL, wyświetl pasek postępu i domyślnie użyj HTTP/2
- `wcurl --curl-options "--clobber --continue-at -" https://example.com/index.html` : wznów przerwane pobieranie

## Links
- More information: https://curl.se/wcurl/manual.html
