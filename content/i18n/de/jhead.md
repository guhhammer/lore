# jhead

Manipulation der Zeitstempel und EXIF Daten in Bilddateien.

## Usage
- `jhead pfad/zu/bild.jpg` : zeige alle EXIF Daten eines JPEG Bildes
- `jhead -ft pfad/zu/bild.jpg` : setze die Dateizeit auf die EXIF Zeit (d.h. die Dateizeit wird geändert)
- `jhead -dsft pfad/zu/bild.jpg` : setze die EXIF Zeit auf die Dateizeit (d.h. die EXIF Zeit wird geändert)
- `jhead -n%Y_%m_%d-%H_%M_%S *.jpg` : benenne alle Dateinamen der JPEG Bilder nach EXIF Datum/Zeit um (Format: Jahr_Monat_Tag-Stunde_Minute_Sekunde.jpg)
- `jhead -autorot *.jpg` : rotiere alle Bilder im aktuellen Ordner verlustfrei (Basierend auf dem EXIF Orientierungstag)
- `jhead -ta-1:00:00 *.jpg` : korrigiere den EXIF Zeitstempel (Format: Stunden:Minuten:Sekunden). Im Beispiel wird jedes Bild eine Stunde in die Vergangenheit geschoben (z.B. Zeitumstellung vergessen)
- `jhead -purejpg pfad/zu/bild.jpg` : entferne alle Metadaten einschließlich der Vorschaubilder aus der JPEG Datei

## Links
- More information: https://www.sentex.net/~mwandel/jhead/usage.html
