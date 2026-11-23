# awk

Wszechstronny język programowania do pracy na plikach.

## Usage
- `awk '{print $5}' ścieżka/do/pliku` : wypisz piątą kolumnę (tzw. pole) w pliku oddzielonym spacjami
- `awk '/foo/ {print $2}' ścieżka/do/pliku` : wypisz drugą kolumnę wierszy zawierających "foo" w pliku oddzielonym spacjami
- `awk -F ',' '{print $NF}' ścieżka/do/pliku` : wypisz ostatnią kolumnę każdego wiersza w pliku, używając przecinka (zamiast spacji) jako separatora pola
- `awk '{s+=$1} END {print s}' ścieżka/do/pliku` : zsumuj wartości w pierwszej kolumnie pliku i wypisz łączną wartość
- `awk 'NR%3==1' ścieżka/do/pliku` : wypisuj co trzeci wiersz, zaczynając od pierwszego
- `awk '{if ($1 == "foo") print "Dokładne dopasowanie foo"; else if ($1 ~ "bar") print "Częściowe dopasowanie bar"; else print "Baz"}' ścieżka/do/pliku` : wypisz różne wartości w zależności od warunków
- `awk '($10 >= min_wartość && $10 <= maks_wartość)'` : wypisz wszystkie linie gdzie wartość 10-tej kolumny jest pomiędzy podanymi wartościami
- `awk 'BEGIN {FS=":";printf "%-20s %6s %25s\n", "Name", "UID", "Shell"} $4 >= 1000 {printf "%-20s %6d %25s\n", $1, $4, $7}' /etc/passwd` : wypisz tabelę użytkowników z UID >=1000 z nagłówkiem i sformatowanym wyjściem, używając dwukropka jako separatora (`%-20s` oznacza: 20 znaków ciągu wyrównanych do lewej, `%6s` oznacza: 6 znaków ciągu wyrównanych do prawej)

## Links
- More information: https://github.com/onetrueawk/awk
