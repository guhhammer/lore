# gpg

GNU Privacy Guard.

## Usage
- `gpg --full-gen-key` : erstelle einen öffentlichen und privaten GPG Schlüssel interaktiv
- `gpg --clearsign doc.txt` : signiere `doc.txt` ohne Verschlüsselung (Ausgabe nach `doc.txt.asc`)
- `gpg -es -r alice@example.com -r bob@example.com doc.txt` : verschlüssle und signiere `doc.txt` für alice@example.com und bob@example.org (Ausgabe nach `doc.txt.gpg`)
- `gpg -c doc.txt` : verschlüssle `doc.txt` nur mit Passwort (Ausgabe nach `doc.txt.gpg`)
- `gpg -d doc.txt.gpg` : entschlüssle `doc.txt.gpg` (Ausgabe nach `stdout`)
- `gpg --import schlüssel.gpg` : importiere einen öffentlichen Schlüssel
- `gpg --export -a alice@example.com` : exportiere den öffentlichen Schlüssel von alice@example.com (Ausgabe nach `stdout`)
- `gpg --export-secret-keys -a alice@example.com` : exportiere den privaten Schlüssel von alice@example.com (Ausgabe nach `stdout`)

## Links
- More information: https://gnupg.org/documentation/manuals/gnupg/Invoking-GPG.html
