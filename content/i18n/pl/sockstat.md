# sockstat

Wyświetl listę otwartych gniazd internetowych lub UNIX-owych.

## Usage
- `sockstat -l` : zobacz, którzy użytkownicy/procesy nasłuchują na których portach
- `sockstat -4|6 -l -P tcp|udp|sctp|divert -p port1,port2...` : pokaż informacje o gniazdach IPv[4]/IPv[6] nasłuchujących (z ang. listening) na określonych portach używając określonego Protokołu
- `sockstat -cnw` : pokaż również połączone (z ang. connected) gniazda, nie rozwiązując numerycznych identyfikatorów UID do nazw użytkowników i używając szerszego (z ang. wider) rozmiaru pola
- `sockstat -jv` : pokaż tylko gniazda dla konkretnego jail-a (ID/nazwa) w trybie informacji pełnej (z ang. verbose mode)
- `sockstat -sU` : wyświetl stan protokołu i numer zdalnego portu enkapsulacji UDP, jeśli dotyczy (obecnie zaimplementowane tylko dla SCTP i TCP)
- `sockstat -CS` : wyświetl moduł kontroli przeciążenia (z ang. Congestion) i Stos protokołu, jeśli dotyczy (obecnie zaimplementowane tylko dla TCP)
- `sockstat -L` : pokaż tylko gniazda internetowe, których adresy (lokalne i zewnętrzne) nie należą do sieci pętli zwrotnej 127.0.0.0/8 ani nie zawierają adresu IPv6 ::1
- `sockstat -qui` : nie pokazuj nagłówka (tryb cichy (z ang. quiet)), pokazując gniazda unix i wyświetlając `inp_gencnt`

## Links
- More information: https://man.freebsd.org/cgi/man.cgi?query=sockstat
