# rsync

Przesyłaj pliki do lub ze zdalnego hosta (ale nie pomiędzy dwoma zdalnymi hostami), domyślnie używając SSH. Aby wskazać na ścieżkę zdalną, użyj `user@host:ścieżka/do/pliku_lub_katalogu`.

## Usage
- `rsync ścieżka/do/źródła ścieżka/do/miejsca_docelowego` : prześlij plik
- `rsync -a ścieżka/do/źródła ścieżka/do/miejsca_docelowego` : użyj trybu archiwum (rekursywnie kopiuj katalogi, kopiuj dowiązania symboliczne bez rozwiązywania i zachowaj uprawnienia, własność i czasy modyfikacji)
- `rsync -zvhP ścieżka/do/źródła ścieżka/do/miejsca_docelowego` : kompresuj dane podczas gdy są wysyłane do miejsca docelowego, wyświetlaj szczegółowy i czytelny dla człowieka postęp i zachowaj częściowo przesłane pliki w przypadku przerwania
- `rsync -r ścieżka/do/źródła ścieżka/do/miejsca_docelowego` : rekursywnie kopiuj katalogi
- `rsync -r ścieżka/do/źródła/ ścieżka/do/miejsca_docelowego` : prześlij zawartość katalogu, ale nie sam katalog
- `rsync -auL ścieżka/do/źródła ścieżka/do/miejsca_docelowego` : rekursywnie kopiuj katalogi, użyj trybu archiwum, rozwiąż dowiązania symboliczne i pomiń pliki, które są nowsze w miejscu docelowym
- `rsync -r --delete rsync://host:ścieżka/do/źródła ścieżka/do/miejsca_docelowego` : prześlij katalog ze zdalnego hosta, na którym działa `rsyncd` i usuń pliki w miejscu docelowym, które nie istnieją w źródle
- `rsync -e 'ssh -p port' --info=progress2 host:ścieżka/do/źródła ścieżka/do/miejsca_docelowego` : prześlij plik poprzez SSH używając innego portu niż domyślny (22) i wyświetlaj globalny postęp

## Links
- More information: https://download.samba.org/pub/rsync/rsync.1
