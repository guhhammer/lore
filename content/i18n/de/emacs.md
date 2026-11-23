# emacs

Der erweiterbare, veränderbare und selbst-dokumentierende Echtzeit Text Editor.

## Usage
- `emacs pfad/zu/datei` : öffne eine Datei in Emacs
- `emacs +zeilennummer pfad/zu/datei` : öffne eine Datei in einer bestimmten Zeile
- `emacs -nw` : starte Emacs in der Konsole (ohne X-Fenster)
- `emacs --daemon` : starte einen Emacs-Server im Hintergrund (aufrufbar mit `emacsclient`)
- `emacsclient --eval '(save-buffers-kill-emacs)'` : beende einen laufenden Emacs-Server und alle Instanzen und frage nach Bestätigung für ungespeicherte Dateien
- `<Ctrl x><Ctrl s>` : tastenkombination zum Speichern einer Datei
- `<Ctrl x><Ctrl c>` : tastenkombination zum Beenden von Emacs

## Links
- More information: https://www.gnu.org/software/emacs
