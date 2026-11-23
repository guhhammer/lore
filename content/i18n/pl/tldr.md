# tldr

Wyświetl proste strony pomocy dla narzędzi wiersza poleceń z projektu tldr-pages. Uwaga: opcje `--language` i `--list` nie są wymagane przez specyfikację, ale większość klientów je implementuje.

## Usage
- `tldr komenda` : wyświetl stronę tldr dla podanej komendy (wskazówka: w ten sposób tu trafiłeś/aś!)
- `tldr komenda podkomenda` : wyświetl stronę tldr dla podanej podkomendy
- `tldr -L kod_języka komenda` : wyświetl stronę tldr dla komendy w podanym języku (jeżeli jest dostępna, w przeciwnym razie po angielsku)
- `tldr -p android|cisco-ios|common|dos|freebsd|linux|netbsd|openbsd|osx|sunos|windows komenda` : wyświetl stronę tldr dla komendy z podanej platformy
- `tldr -u` : zaktualizuj lokalną pamięć podręczną stron tldr
- `tldr -l` : wyświetl listę stron tldr dla aktualnej platformy i `common`
- `tldr -l | fzf --preview "tldr {1} --color=always" --preview-window=right,70% | xargs tldr` : przeglądaj strony tldr w oknie terminala (`fzf` musi być zainstalowany)
- `tldr -l | shuf -n 1 | xargs tldr` : wyświetl stronę tldr dla losowej komendy
