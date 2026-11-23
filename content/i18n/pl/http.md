# http

HTTPie: klient HTTP przeznaczony do testowania, debugowania i ogólnej interakcji z API i serwerami HTTP.

## Usage
- `http https://example.com` : wykonaj proste żądanie GET (wyświetla nagłówki odpowiedzi i zawartość)
- `http -p H|B|h|b|m|Hh|Hhb|... https://example.com` : wyświetl podane części treści (`H`: nagłówki żądania, `B`: treść żądania, `h`: nagłówki odpowiedzi, `b`: treść odpowiedzi, `m`: metadane odpowiedzi)
- `http GET|POST|HEAD|PUT|PATCH|DELETE|... --proxy http|https:http://localhost:8080|socks5://localhost:9050|... https://example.com` : określ metodę HTTP używaną podczas wysyłania żądania i użyj serwera proxy do przechwycenia żądania
- `http -F https://example.com 'User-Agent: Mozilla/5.0' 'Accept-Encoding: gzip'` : podążaj za wszystkimi przekierowaniami `3xx` i określ dodatkowe nagłówki do żądania
- `http -a nazwa_użytkownika:hasło|token -A basic|digest|bearer GET|POST|... https://example.com/auth` : uwierzytelnij się na serwerze używając różnych metod uwierzytelniania
- `http --offline GET|DELETE|... https://example.com` : skonstruuj żądanie, ale go nie wysyłaj
- `http --session nazwa_sesji|ścieżka/do/sesji.json -a nazwa_użytkownika:hasło https://example.com/auth API-KEY:xxx` : użyj nazwanych sesji do trwałych niestandardowych nagłówków, danych uwierzytelniających i ciasteczek
- `http -f POST https://example.com/upload cv@ścieżka/do/pliku` : prześlij plik do formularza (poniższy przykład zakłada, że polem formularza jest `<input type="file" name="cv" />`)

## Links
- More information: https://httpie.io/docs/cli/usage
