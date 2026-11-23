# adb install

Push pakketten naar een aangesloten Android-apparaat of -emulator.

## Usage
- `adb install pad/naar/bestand.apk` : push een Android-applicatie naar een emulator/apparaat
- `adb -s serienummer install pad/naar/bestand.apk` : push een Android-applicatie naar een specifieke emulator/apparaat (heeft voorrang op `$ANDROID_SERIAL`)
- `adb install -r pad/naar/bestand.apk` : installeer een bestaande app opnieuw, waarbij de gegevens behouden blijven
- `adb install -d pad/naar/bestand.apk` : push een Android-applicatie en sta downgrade van de versiecode toe (alleen foutopsporingspakketten)
- `adb install -g pad/naar/bestand.apk` : verleen (g) alle machtigingen die worden vermeld in het app-manifest
- `adb install --fastdeploy pad/naar/bestand.apk` : werk snel een geïnstalleerd pakket bij door alleen de delen van de APK bij te werken die zijn gewijzigd

## Links
- More information: https://developer.android.com/tools/adb
