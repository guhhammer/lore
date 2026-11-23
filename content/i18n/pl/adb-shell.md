# adb shell

Android Debug Bridge Shell: uruchamiaj zdalne polecenia powłoki na instancji emulatora Androida lub podłączonych urządzeniach z Androidem.

## Usage
- `adb shell` : uruchom interaktywną zdalną powłokę na emulatorze lub urządzeniu
- `adb shell getprop` : pobierz wszystkie właściwości z emulatora lub urządzenia
- `adb shell pm reset-permissions` : przywróć domyślne ustawienia wszystkich uprawnień uruchamiania
- `adb shell pm revoke pakiet pozwolenie` : odwołaj niebezpieczne pozwolenie dla aplikacji
- `adb shell input keyevent kod_klawisza` : wywołaj zdarzenie klawisza
- `adb shell pm clear pakiet` : wyczyść dane aplikacji na emulatorze lub urządzeniu
- `adb shell am start -n pakiet/aktywność` : rozpocznij aktywność na emulatorze lub urządzeniu
- `adb shell am start -W -c android.intent.category.HOME -a android.intent.action.MAIN` : rozpocznij aktywność domową na emulatorze lub urządzeniu

## Links
- More information: https://developer.android.com/tools/adb
