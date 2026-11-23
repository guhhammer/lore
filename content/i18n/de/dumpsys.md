# dumpsys

Stelle Informationen über Android-Systemservices bereit. Dieser Befehl kann nur mit `adb shell` verwendet werden.

## Usage
- `dumpsys` : erhalte diagnostische Informationen über alle Systemservices
- `dumpsys service` : erhalte diagnostische Informationen über einen bestimmten Systemservice
- `dumpsys -l` : liste alle Services, über die `dumpsys` Informationen bereitstellen kann auf
- `dumpsys service -h` : liste Service-spezifische Argumente für einen Service auf
- `dumpsys --skip service` : schließe einen bestimmten Service von den diagnostischen Informationen aus
- `dumpsys -t sekunden` : gib ein Timeout in Sekunden an (standardmäßig 10s)

## Links
- More information: https://developer.android.com/tools/dumpsys
