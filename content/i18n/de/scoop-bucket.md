# scoop bucket

Verwalte "Eimer": Git-Repositories, welche Dateien enthalten, die beschreiben, wie Scoop Programme installiert werden. Kennt Scoop nicht die URL eines Eimers, so muss diese angegeben werden.

## Usage
- `scoop bucket list` : liste alle Eimer auf, die gerade aktiv sind
- `scoop bucket known` : liste alle bekannten Eimer auf
- `scoop bucket add name` : aktiviere einen bekannten Eimer
- `scoop bucket add name https://example.com/repository.git` : aktiviere einen unbekannten Eimer durch die Angabe eines Namens und einer Git-Repository-URL
- `scoop bucket rm name` : deaktiviere einen Eimer

## Links
- More information: https://github.com/ScoopInstaller/Scoop/wiki/Buckets
