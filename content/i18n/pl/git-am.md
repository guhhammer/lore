# git am

Zastosuj pliki poprawki i utwórz commit. Przydatne przy otrzymywaniu commitów przez email.

## Usage
- `git am ścieżka/do/pliku.patch` : zastosuj i komituj zmiany z lokalnego pliku poprawki
- `curl -L https://example.com/file.patch | git am` : zastosuj i komituj zmiany ze zdalnego pliku poprawki
- `git am --abort` : przerwij proces zastosowania pliku poprawki
- `git am --reject ścieżka/do/pliku.patch` : zastosuj jak największą część pliku poprawki, zapisując nieudane fragmenty w plikach odrzuceń (pliki `.rej`)

## Links
- More information: https://git-scm.com/docs/git-am
