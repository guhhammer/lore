# logcat

Gib ein Protokoll aller System-Logs aus.

## Usage
- `logcat` : gib ein Protokoll aller System-Logs aus
- `logcat -f pfad/zu/datei` : schreibe alle System-Logs in eine Datei
- `logcat --regex regex` : gib Zeilen aus, die einem regulären Ausdruck entsprechen
- `logcat --pid pid` : gib System-Logs für die spezifizierte PID aus
- `logcat --pid $(pidof -s packet)` : gib System-Logs für den Prozess eines bestimmten Packets aus

## Links
- More information: https://developer.android.com/tools/logcat
