# dumpsys

Geef informatie over Android-systeemservices. Dit commando kan alleen worden gebruikt via `adb shell`.

## Usage
- `dumpsys` : krijg diagnostische output voor alle systeemservices
- `dumpsys service` : krijg diagnostische output voor een specifieke systeemservice
- `dumpsys -l` : toon alle services waar `dumpsys` informatie over kan geven
- `dumpsys service -h` : maak een lijst van servicespecifieke argumenten voor een service
- `dumpsys --skip service` : sluit een specifieke service uit van de diagnostische output
- `dumpsys -t 8` : geef een timeoutperiode in seconden op (standaard 10s)

## Links
- More information: https://developer.android.com/tools/dumpsys
