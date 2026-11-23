# adb reverse

Android Debug Bridge Reverse: zwrotne połączenie socketowe z emulowanego lub prawdziwego urządzenia Android.

## Usage
- `adb reverse --list` : wypisz wszystkie zwrotne połączenia socketowe z emulatorów i urządzeń
- `adb reverse tcp:zdalny_port tcp:lokalny_port` : przekieruj port TCP z emulatora lub urządzenia do localhost
- `adb reverse --remove tcp:zdalny_port` : usuń wybrane zwrotne połączenie z emulatora lub urządzenia
- `adb reverse --remove-all` : usuń wszystkie zwrotne połączenia z wszystkich emulatorów lub urządzeń

## Links
- More information: https://developer.android.com/tools/adb
