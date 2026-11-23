# nmap

Narzędzie do eksploracji sieci oraz skaner bezpieczeństwa/portów. Niektóre funkcje (np. skan SYN) aktywują się tylko gdy `nmap` jest uruchamiany z przywilejami root'a.

## Usage
- `nmap -v1|2|3 ip_lub_nazwa_hosta` : skanuj 1000 najpopularniejszych portów zdalnego hosta z różnymi poziomami szczegółowości (verbosity)
- `nmap -T5 -sn 192.168.0.0/24|ip_lub_nazwa_hosta1,ip_lub_nazwa_hosta2,...` : wykonaj bardzo agresywnie ping sweep w całej podsieci lub na poszczególnych hostach
- `sudo nmap -A -iL ścieżka/do/pliku.txt` : włącz wykrywanie systemu operacyjnego, wykrywanie wersji, skanowanie skryptów i traceroute hostów z pliku
- `nmap -p port1,port2,... ip_lub_host1,ip_lub_host2,...` : skanuj podaną listę portów (użyj `-p-` dla wszystkich portów od 1 do 65535)
- `nmap -sC -sV -oA top-1000-ports ip_lub_host1,ip_lub_host2,...` : przeprowadź wykrywanie usług i wersji dla 1000 najpopularniejszych portów używając domyślnych skryptów NSE, zapisując wynik (`-oA`) do plików wyjściowych
- `nmap --script "default and safe" ip_lub_host1,ip_lub_host2,...` : skanuj cel(e) ostrożnie używając skryptów NSE `default and safe`
- `nmap --script "http-*" ip_lub_host1,ip_lub_host2,... -p 80,443` : skanuj w poszukiwaniu serwerów internetowych działających na standardowych portach 80 i 443 przy użyciu wszystkich dostępnych skryptów NSE `http-*`
- `sudo nmap -T0 -D ip_wabika1,ip_wabika2,... --source-port 53 -f --data-length 16 -Pn ip_lub_host` : spróbuj uniknąć wykrycia przez IDS/IPS, używając bardzo powolnego skanowania (`-T0`), fałszywych adresów źródłowych - wabików (`-D`), fragmentowanych pakietów, losowych danych i innych metod

## Links
- More information: https://nmap.org/book/man.html
