# wsl

Verwalte das Windows Subsystem für Linux von der Kommandozeile.

## Usage
- `wsl shell_befehl` : starte eine Linux-Shell (in der Standard-Distribution)
- `wsl -e befehl befehl_argumente` : führe einen Linux-Befehl aus, ohne eine Shell zu benutzen
- `wsl -d distribution shell_befehl` : gib eine bestimmte Distribution an
- `wsl -l` : liste verfügbare Distributionen auf
- `wsl --export distribution pfad\zu\datei.tar` : exportiere eine Distribution in eine `.tar` Datei
- `wsl --import distribution pfad\zu\installations_verzeichnis pfad\zu\datei.tar` : importiere eine Distribution von einer `.tar` Datei
- `wsl --set-version distribution version` : ändere die WSL-Version einer bestimmten Distribution
- `wsl --shutdown` : fahre das Windows Subsystem für Linux herunter

## Links
- More information: https://learn.microsoft.com/windows/wsl/reference
