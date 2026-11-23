# arduino

Arduino Studio - Environnement de Développement Intégré pour la plateforme Arduino.

## Usage
- `arduino --verify chemin/vers/croquis.ino` : construis un croquis
- `arduino --upload chemin/vers/croquis.ino` : construis et téléverse un croquis
- `arduino --board arduino:avr:nano:cpu=atmega328p --port /dev/ttyACM0 --upload chemin/vers/croquis.ino` : construis et téléverse un croquis vers un Arduino Nano avec un CPU Atmega328p, connecté sur le port `/dev/ttyACM0`
- `arduino --pref nom=valeur` : configure la préférence `nom` à une valeur `valeur`
- `arduino --pref build.path=chemin/vers/dossier/de/construction --verify chemin/vers/croquis.ino` : construis un croquis, mets le résultat de ce dernier dans un dossier, et réutilise n'importe quelles versions précédentes dans ce dossier
- `arduino --save-prefs` : sauvegarde toutes préférences (modifiées) dans un fichier `preferences.txt`

## Links
- More information: https://github.com/arduino/Arduino/blob/master/build/shared/manpage.adoc
