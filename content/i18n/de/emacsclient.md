# emacsclient

Öffnet Dateien in einem laufenden Emacs-Server.

## Usage
- `emacsclient pfad/zu/datei` : öffne eine Datei in einem laufenden Emacs-Server (mit GUI wenn möglich)
- `emacsclient -nw pfad/zu/datei` : öffne eine Datei in der Konsole (ohne X-Fenster)
- `emacsclient -c pfad/zu/datei` : öffne eine Datei in einem neuen Emacs Fenster
- `emacsclient -e '(befehl)'` : führe einen Befehl aus und schreibe das Ergebnis in `stdout`
- `emacsclient -a editor pfad/zu/datei` : gib einen alternativen Editor an für den Fall, dass kein Emacs-Server läuft
- `emacsclient -e '(save-buffers-kill-emacs)'` : beende einen laufenden Emacs-Server und alle Instanzen und frage nach Bestätigung für ungespeicherte Dateien
