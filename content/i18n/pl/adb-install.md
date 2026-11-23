# adb install

Android Debug Bridge Install: wysyłaj pakiety do instancji emulatora Androida lub podłączonych urządzeń z systemem Android.

## Usage
- `adb install ścieżka/do/pliku.apk` : wyślij aplikację na Androida do emulatora/urządzenia
- `adb -s numer_seryjny install ścieżka/do/pliku.apk` : wyślij aplikację Android do określonego emulatora/urządzenia (nadpisuje `$ANDROID_SERIAL`)
- `adb install -r ścieżka/do/pliku.apk` : zainstaluj ponownie (reinstall) istniejącą aplikację, zachowując jej dane
- `adb install -d ścieżka/do/pliku.apk` : wyślij aplikację na Androida, umożliwiając obniżenie (downgrade) wersji kodu (tylko pakiety debugowalne)
- `adb install -g ścieżka/do/pliku.apk` : przyznaj (grant) wszystkie uprawnienia wymienione w pliku manifestu aplikacji
- `adb install --fastdeploy ścieżka/do/pliku.apk` : szybko zaktualizuj zainstalowany pakiet, aktualizując tylko te części APK, które się zmieniły

## Links
- More information: https://developer.android.com/tools/adb
