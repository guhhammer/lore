# aspell

Interaktiver Korrekturleser. Weitere Informationen: <http://aspell.net/man-html/index.html>.

## Usage
- `aspell check pfad/zu/datei` : überprüfe eine einzelne Datei auf Fehler
- `cat pfad/zu/datei | aspell list` : liste falsch geschriebene Worte von `stdin`
- `aspell dicts` : zeige verfügbare Wörterbuchsprachen
- `aspell --lang cs` : nutze `aspell` mit einem anderen Wörterbuch (nimmt 2-Zeichen-Locale laut ISO 639 Sprach Code)
- `cat pfad/zu/datei | aspell --personal persönliche-wort-liste.pws list` : zeige alle falsch geschriebenen Wörter von `stdin` und ignoriere alle Wörter in einer persönlichen Wortliste
