# kiterunner wordlist

Een contextuele webscanner voor het beheren van woordenlijsten die gebruikt worden bij het ontdekken van API-paden en webeindpunten. Het `wordlist` subcommando zorgt voor het opsommen en opslaan van woordenlijsten in `~/.cache/kiterunner`.

## Usage
- `kiterunner wordlist list` : maak een lijst van alle in de cache opgeslagen en beschikbare Assetnote woordenlijsten
- `kiterunner wordlist list -o json` : geef woordenlijsten met JSON-uitvoer weer
- `kiterunner wordlist list -v debug` : geef woordenlijsten met uitgebreide debug-uitvoer weer
- `kiterunner wordlist save apiroutes-210328` : sla een specifieke Assetnote woordenlijst op met een alias
- `kiterunner wordlist save pad/naar/httparchive_apiroutes_2024_05_28.txt` : sla een specifieke Assetnote woordenlijst op met de volledige bestandsnaam
- `kiterunner wordlist save apiroutes-210328,aspx-210328` : sla meerdere woordenlijsten op met een alias
- `kiterunner wordlist save apiroutes-210328 -q` : sla een woordenlijst op met stille modus om uitvoer te onderdrukken

## Links
- More information: https://github.com/assetnote/kiterunner#usage
