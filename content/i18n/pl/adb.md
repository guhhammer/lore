# adb

Android Debug Bridge: komunikuj się z instancją emulatora Androida lub podłączonym urządzeniem z Androidem. Niektóre podkomendy takie jak `shell` mają osobną dokumentację.

## Usage
- `adb start-server` : sprawdź czy proces serwera adb działa, jeśli nie, uruchom go
- `adb kill-server` : zakończ proces serwera adb
- `adb shell` : uruchom powłokę w docelowej instancji emulatora/urządzenia
- `adb install -r ścieżka/do/pliku.apk` : wyślij aplikację Android do emulatora/urządzenia
- `adb pull ścieżka/do/pliku_lub_katalogu_na_urządzeniu ścieżka/do/lokalnego_katalogu_docelowego` : skopiuj plik/katalog z urządzenia docelowego
- `adb push ścieżka/do/lokalnego_pliku_lub_katalogu ścieżka/do/docelowego_katalogu_na_urządzeniu` : skopiuj plik/katalog do urządzenia docelowego
- `adb devices` : wypisz wszystkie połączone urządzenia

## Links
- More information: https://developer.android.com/tools/adb
