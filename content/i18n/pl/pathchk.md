# pathchk

Sprawdź poprawność oraz przenośność jednej lub większej ilości ścieżek.

## Usage
- `pathchk ścieżka1 ścieżka2 ...` : sprawdź ścieżki pod kątem poprawności w obecnym systemie
- `pathchk -p ścieżka1 ścieżka2 ...` : sprawdź ścieżki pod kątem poprawności w szerszym zakresie systemów zgodnych z POSIX
- `pathchk -p -P ścieżka1 ścieżka2 ...` : sprawdź ścieżki pod kątem poprawności we wszystkich systemach zgodnych z POSIX
- `pathchk -P ścieżka1 ścieżka2 ...` : sprawdź tylko pod kątem pustych ścieżek lub wiodących myślników (-)
