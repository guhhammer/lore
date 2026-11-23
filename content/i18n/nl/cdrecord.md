# cdrecord

Schrijf gegevens naar cd's of dvd's. Sommige aanroepen van cdrecord kunnen destructieve acties veroorzaken, zoals het wissen van alle gegevens op een schijf.

## Usage
- `cdrecord --devices` : toon optische stations die beschikbaar zijn voor `cdrecord`
- `cdrecord dev=/dev/optisch_station -audio track*.cdaudio` : neem een schijf met alleen audio op ("branden")
- `cdrecord -eject dev=/dev/optisch_station -data bestand.iso` : brand een bestand naar een schijf en werp de schijf uit zodra dit klaar is (sommige recorders vereisen dit)
- `cdrecord -tao dev=/dev/optisch_station -data bestand.iso` : brand een bestand naar de schijf in een optisch station, mogelijk naar meerdere schijven achter elkaar

## Links
- More information: https://manned.org/cdrecord
