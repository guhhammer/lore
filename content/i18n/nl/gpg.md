# gpg

GNU Privacy Guard, een OpenPGP encryptie- en ondertekeningstool.

## Usage
- `gpg --full-gen-key` : maak interactief een GPG publieke en privésleutel
- `gpg -k` : geef alle sleutels van de publieke sleutelring weer
- `gpg --clearsign doc.txt` : onderteken `doc.txt` zonder encryptie (schrijft uitvoer naar `doc.txt.asc`)
- `gpg -es -r alice@example.com -r bob@example.com doc.txt` : versleutel en onderteken `doc.txt` voor `alice@example.com` en `bob@example.com` (schrijft uitvoer naar `doc.txt.gpg`)
- `gpg -c doc.txt` : versleutel `doc.txt` met alleen een wachtwoordzin (uitvoer naar `doc.txt.gpg`)
- `gpg -d doc.txt.gpg` : ontcijfer `doc.txt.gpg` (uitvoer naar `stdout`)
- `gpg --locate-keys alice@example.com` : importeer een publieke sleutel vanaf WKD (Web Key Directory) als de sleutel nog niet in de sleutelring zit
- `gpg --export|--export-secret-keys -a alice@example.com` : exporteer de publieke/privésleutel voor `alice@example.com` (uitvoer naar `stdout`)

## Links
- More information: https://gnupg.org/documentation/manuals/gnupg/Invoking-GPG.html
