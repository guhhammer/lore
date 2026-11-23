# snoop

Sniffer pakietów sieciowych. Odpowiednik `tcpdump` w systemie SunOS.

## Usage
- `snoop -d e1000g0` : przechwyć pakiety na określonym interfejsie sieciowym
- `snoop -o ścieżka/do/pliku` : zapisz przechwycone pakiety w pliku zamiast ich wyświetlania
- `snoop -V -i ścieżka/do/pliku` : wyświetl szczegółowe podsumowanie warstwy protokołu pakietów z pliku
- `snoop to port port from host nazwa_hosta` : przechwyć pakiety sieciowe, które pochodzą z nazwy hosta i trafiają na dany port
- `snoop -x0 -p4 ip1 ip2` : przechwyć i wyświetl zrzut heksadecymalny pakietów sieciowych wymienianych między dwoma adresami IP

## Links
- More information: https://www.unix.com/man-page/sunos/1m/snoop
