# grep

Findet Ausdrücke in einem Eingabetext. Unterstützt einfache Muster und reguläre Ausdrücke.

## Usage
- `grep "ausdruck" pfad/zu/datei` : suche nach einem Ausdruck in einer Datei
- `grep -F "exakter_ausdruck" pfad/zu/datei` : suche nach einem exakten Ausdruck
- `grep -Ei "ausdruck" pfad/zu/datei` : benutze erweiterte reguläre Ausdrücke (unterstützt `?`, `+`, `{}`, `()` und `|`) ohne Beachtung der Groß-, Kleinschreibung
- `grep --context|--before-context|--after-context 3 "ausdruck" pfad/zu/datei` : zeige 3 Zeilen Kontext um C, vor B oder nach A jedem Ergebnis

## Links
- More information: https://www.gnu.org/software/grep/manual/grep.html
