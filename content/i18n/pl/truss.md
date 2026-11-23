# truss

Narzędzie do rozwiązywania problemów poprzez śledzenie wywołań systemowych. Odpowiednik strace w SunOS.

## Usage
- `truss -f program` : rozpocznij śledzenie programu, wykonując go i śledząc wszystkie procesy potomne
- `truss -p pid` : rozpocznij śledzenie określonego procesu według jego PID
- `truss -a -e program` : rozpocznij śledzenie programu, wykonując go, pokazując argumenty i zmienne środowiskowe
- `truss -c -p pid` : zlicz czas, wywołania i błędy dla każdego wywołania systemowego i raportuj podsumowanie po zakończeniu programu
- `truss -p pid -t nazwa_wywołania_systemowego` : śledź proces filtrując dane wyjściowe według wywołania systemowego

## Links
- More information: https://www.unix.com/man-page/sunos/1/truss
