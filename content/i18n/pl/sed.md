# sed

Edytuj tekst w sposób skryptowalny.

## Usage
- `komenda | sed 's/jabłko/mango/g'` : zamień wszystkie wystąpienia `jabłko` (podstawowe `regex`) na `mango` (podstawowe `regex`) we wszystkich liniach wejściowych i wypisz wynik do `stdout`
- `komenda | sed -f ścieżka/do/skryptu.sed` : uruchom określony plik (z ang. file) skryptu i wydrukuj wynik na `stdout`
- `komenda | sed -af ścieżka/do/skryptu.sed` : opóźnij otwieranie każdego pliku, dopóki polecenie zawierające powiązaną funkcję lub flagę `w` nie zostanie zastosowane do linii wejściowej
- `komenda | sed -E 's/(jabłko)/\U\1/g'` : zamień wszystkie wystąpienia `jabłko` (rozszerzone `regex`) na `JABŁKO` (rozszerzone `regex`) we wszystkich liniach wejściowych i wypisz wynik do `stdout`
- `komenda | sed -n '1p'` : wypisz tylko pierwszą linię do `stdout`
- `sed -i 's/jabłko/mango/g' ścieżka/do/skryptu` : zamień wszystkie wystąpienia `jabłko` (podstawowe `regex`) na `mango` (podstawowe `regex`) w określonym pliku i nadpisz oryginalny plik

## Links
- More information: https://man.freebsd.org/cgi/man.cgi?query=sed
