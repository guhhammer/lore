# masscan

Bardzo efektywny skaner sieci. Najlepiej działa z podwyższonymi uprawnieniami. Aby uzyskać pomoc dotyczącą zgodności z Nmap'em, użyj komendy `masscan --nmap`.

## Usage
- `masscan adres_ip|maska_podsieci -p 80` : skanuj adres IP lub podsieć w poszukowaniu portu 80
- `masscan 10.0.0.0/16 --top-ports 100 --rate 100000` : skanuj podsieć klasy B w poszukiwaniu 100 najczęstszych portów z prędkością 100 000 pakietów na sekundę
- `masscan 10.0.0.0/16 --top-ports 100 --excludefile ścieżka/do/pliku` : skanuj podsieć klasy B unikając zakresów podanych w pliku
- `masscan 10.0.0.0/16 -p 22,80 --banners --rate 100000` : skanuj podsieć klasy B używając wykrywania wersji podobnego do Nmap'a (banner grabbing)
- `masscan 0.0.0.0/0 -p 80,443 --rate 10000000` : skanuj Internet w poszukiwaniu serwerów WWW działających na portach 80 i 443
- `masscan 0.0.0.0/0 -p U:53 --rate 10000000` : skanuj Internet w poszukiwaniu serwerów DNS działających na porcie UDP 53
- `masscan 0.0.0.0/0 -p 0-65535 --output-format binary|grepable|json|list|xml --output-filename path/to/file` : skanuj Internet w poszukiwaniu podanego zakresu portów i eksportuj wynik do pliku
- `masscan --readscan path/to/file` : odczytaj binarny wynik skanu z pliku i wypisz do `stdout`

## Links
- More information: https://manned.org/masscan
