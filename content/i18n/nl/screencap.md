# screencap

Maak een screenshot van een mobiel scherm. Opmerking: dit commando kan alleen worden gebruikt via `adb shell`.

## Usage
- `screencap pad/naar/afbeelding.png` : maak een screenshot
- `screencap -p` : toon bestandsinhoud naar `stdout` als PNG
- `adb shell screencap -p >pad/naar/afbeelding.png` : neem een screenshot en sla deze op via een `adb`-verbinding
- `screencap -h` : toon de help

## Links
- More information: https://developer.android.com/tools/adb#screencap
