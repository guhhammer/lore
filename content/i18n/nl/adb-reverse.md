# adb reverse

Keer socketverbindingen om van een verbonden Android-apparaat of -emulator.

## Usage
- `adb reverse --list` : toon alle omgekeerde socketverbindingen van emulators en apparaten
- `adb reverse tcp:externe_poort tcp:lokale_poort` : keer een TCP-poort om van de enige verbonden emulator of apparaat naar localhost
- `adb -s apparaat_id reverse tcp:externe_poort tcp:lokale_poort` : keer een TCP-poort om van een specifieke emulator of apparaat (via apparaat-ID of serienummer) naar localhost
- `adb reverse --remove tcp:externe_poort` : verwijder een omgekeerde socketverbinding van een emulator of apparaat
- `adb reverse --remove-all` : verwijder alle omgekeerde socketverbindingen van alle emulators en apparaten

## Links
- More information: https://developer.android.com/tools/adb
