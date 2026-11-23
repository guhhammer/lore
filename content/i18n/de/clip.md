# clip

Kopieren von Inhalten der Befehlsausgabe in die Windows Zwischenablage.

## Usage
- `dir | clip` : kopiere die Ausgabe eines Befehls in die Windows Zwischenablage
- `clip <pfad/zu/datei.txt` : kopiere den Inhalt einer Datei in die Windows Zwischenablage
- `echo irgendein text | clip` : kopiere Text mit einem nachfolgenden Zeilenumbruch in die Windows Zwischenablage
- `echo | set /p="irgendein text" | clip` : kopiere Text ohne nachfolgenden Zeilenumbruch in die Windows Zwischenablage
