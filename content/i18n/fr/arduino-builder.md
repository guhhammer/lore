# arduino-builder

Un outil en ligne de commande pour compiler des croquis arduino. AVERTISSEMENT DE DÉPRÉCIATION: Cet outil a été retiré au profit de `arduino`.

## Usage
- `arduino-builder -compile chemin/vers/croquis.ino` : construit un croquis
- `arduino-builder -debug-level niveau` : spécifie le niveau de débogage (1 à 10, 5 par défaut)
- `arduino-builder -build-path chemin/vers/répertoire/de/construction` : spécifie un dossier de construction
- `arduino-builder -build-options-file chemin/vers/construction.options.json` : utilise un fichier d'option de construction, au lieu de spécifier `-hardware`, `-tools`, etc. manuellement à chaque fois
- `arduino-builder -verbose true` : active le mode verbeux

## Links
- More information: https://github.com/arduino/arduino-builder
