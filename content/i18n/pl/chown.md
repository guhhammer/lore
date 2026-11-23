# chown

Zmienia właściciela i grupę właścicieli dla plików i katalogów.

## Usage
- `chown użytkownik ścieżka/do/pliku_lub_katalogu` : zmień właściciela pliku/katalogu
- `chown użytkownik:grupa ścieżka/do/pliku_lub_katalogu` : zmień właściciela i grupę właścicieli pliku/katalogu
- `chown -R użytkownik ścieżka/do/katalogu` : rekursywnie zmień właściciela katalogu i jego zawartości
- `chown -h użytkownik ścieżka/do/dowiązania_symbolicznego` : zmień właściciela dowiązania symbolicznego
- `chown --reference ścieżka/do/pliku_referencyjnego ścieżka/do/pliku_lub_katalogu` : zmień właściciela pliku/katalogu by był taki sam jak w pliku referencyjnym
