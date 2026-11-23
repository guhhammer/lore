# logcat

Dump een logboek van systeemberichten, inclusief stacktraces wanneer er een fout is opgetreden, en informatieberichten die door applicaties zijn vastgelegd.

## Usage
- `logcat` : toon systeemlogs
- `logcat -f pad/naar/bestand` : schrijf systeemlogs naar een bestand (f)
- `logcat --regex regex` : toon lijnen die overeenkomen met een `regex`
- `logcat --pid proces_id` : toon logs voor een specifieke PID
- `logcat --pid $(pidof -s pakket)` : toon logs voor een proces van een specifiek pakket

## Links
- More information: https://developer.android.com/tools/logcat
