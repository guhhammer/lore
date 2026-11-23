# adb

Android Debug Bridge: communiceer met een Android-emulator of een aangesloten Android-apparaat. Sommige subcommando's zoals `shell` hebben hun eigen documentatie.

## Usage
- `adb start-server` : controleer of het adb-serverproces draait en start het
- `adb kill-server` : sluit het adb-serverproces
- `adb shell` : start een remote shell voor de doel-emulator of apparaat-instantie
- `adb install -r pad/naar/bestand.apk` : push een Android-applicatie naar een emulator/apparaat
- `adb pull pad/naar/extern/bestand_of_map pad/naar/lokaal/bestand_of_map` : kopieer een bestand/map van het doelapparaat
- `adb push pad/naar/lokaal/bestand_of_map pad/naar/extern/bestand_of_map` : kopieer een bestand/map naar het doelapparaat
- `adb devices` : toon alle aangesloten apparaten
- `adb -s apparaat_id shell` : specificeer naar welk apparaat de opdrachten verzonden dienen te worden als er meerdere apparaten zijn

## Links
- More information: https://developer.android.com/tools/adb
