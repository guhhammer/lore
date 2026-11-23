# kiterunner kb

Een contextuele webscanner voor het manipuleren van kitebuilder-schema's die gebruikt worden bij het ontdekken van API-paden en webeindpunten. Het `kb` subcommando zorgt voor schemacompilatie, conversie, parsing en verzoekherhaling.

## Usage
- `kiterunner kb compile pad/naar/woordenlijst.json pad/naar/woordenlijst.kite` : compileer een kitebuilder-schema van JSON naar een kite-bestand
- `kiterunner kb convert pad/naar/woordenlijst.kite pad/naar/woordenlijst.txt` : converteer een kite-bestand naar een tekstwoordenlijst
- `kiterunner kb convert pad/naar/woordenlijst.txt pad/naar/woordenlijst.kite` : converteer een tekstwoordenlijst naar een kite-bestand
- `kiterunner kb convert pad/naar/woordenlijst.kite pad/naar/woordenlijst.json` : converteer een kite-bestand naar een JSON-schema
- `kiterunner kb parse pad/naar/woordenlijst.json -o json` : parseer een kitebuilder-schema en voer opgemaakte JSON-data uit
- `kiterunner kb parse pad/naar/woordenlijst.kite -o text` : parseer een kite-bestand en voer opgemaakte tekstgegevens uit
- `kiterunner kb replay -w pad/naar/woordenlijst.kite "request_output"` : speel een specifiek verzoek van een kitebuilder-schema-uitvoer opnieuw af
- `kiterunner kb replay -w pad/naar/woordenlijst.kite -p http://localhost:8080 "request_output"` : herhaal een verzoek via een proxy voor inspectie

## Links
- More information: https://github.com/assetnote/kiterunner#usage
