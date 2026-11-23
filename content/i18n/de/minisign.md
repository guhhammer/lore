# minisign

Ein denkbar einfaches Werkzeug, um Dateien zu signieren und Signaturen zu verifizieren.

## Usage
- `minisign -G` : generiere ein neues Schlüsselpaar im Standardpfad
- `minisign -Sm pfad/zu/datei` : signiere eine Datei
- `minisign -Sm pfad/zu/datei -c "Nicht vertrauenswürdiger Kommentar" -t "Vertrauenswürdiger Kommentar"` : signiere eine Datei und füge dabei einen vertrauenswürdigen (signierten) und einen nicht vertrauenswürdigen (unsignierten) Kommentar in der Signatur an
- `minisign -Vm pfad/zu/datei -p pfad/zu/öffentlicher_schlüssel.pub` : verifiziere eine Datei und die vertrauenswürdigen Kommentare in ihrer Signatur gegen die angegebene Datei mit dem öffentlichen Schlüssel
- `minisign -Vm pfad/zu/datei -P "öffentlicher_schlüssel_base64"` : verifiziere eine Datei und die vertrauenswürdigen Kommentare in ihrer Signatur gegen den angegebenen, in Base64 codierten öffentlichen Schlüssel

## Links
- More information: https://jedisct1.github.io/minisign/
