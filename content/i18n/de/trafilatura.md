# trafilatura

Ein Python-Tool zum Extrahieren und Crawlen von Webinhalten. Extrahiert Haupttext, Metadaten und Kommentare von Webseiten.

## Usage
- `trafilatura -u url` : extrahiere Text einer Website
- `trafilatura -u url -o pfad/zur/ausgabe.txt` : extrahiere Text und speichere diesen in einer Datei
- `trafilatura -u url --json` : extrahiere Text im JSON-Format
- `trafilatura -i pfad/zur/url_liste.txt` : extrahiere Text von mehreren URLs aus einer Datei
- `trafilatura --sitemap url_zur_sitemap.xml` : crawle eine Website basierend auf einer Sitemap
- `trafilatura -u url --formatting` : extrahiere Text unter Beibehaltung der HTML-Formatierung
- `trafilatura -u url --with-comments` : extrahiere Text inklusive Kommentare
- `trafilatura -h` : zeige Hilfe an
