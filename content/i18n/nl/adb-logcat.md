# adb logcat

Dump een logboek met systeemberichten.

## Usage
- `adb logcat` : geef systeemlogboeken weer
- `adb logcat -e regex` : geef regels weer die overeenkomen met een `regex`
- `adb logcat tag:modus *:S` : toon logs voor een tag in een specifieke modus (Verbose, Debug, Info, Warning, Error, Fatal, Silent), waarbij andere tags worden gefilterd
- `adb logcat ReactNative:V ReactNativeJS:V *:S` : geef logs weer voor React Native-applicaties in Verbose-modus, waarbij andere tags worden onderdrukt (Silent)
- `adb logcat *:W` : toon logs voor alle tags met prioriteitsniveau Warning en hoger
- `adb logcat --pid proces_id` : geef logs weer voor een specifieke PID
- `adb logcat --pid $(adb shell pidof -s pakket)` : geef logs weer voor het proces van een specifiek pakket
- `adb logcat -v color` : kleur de log in (gebruik meestal met filters)

## Links
- More information: https://developer.android.com/tools/logcat
