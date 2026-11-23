# svcs

Geef informatie over actieve services.

## Usage
- `svcs` : toon alle actieve services
- `svcs -vx` : toon inactieve services
- `svcs apache` : geef informatie over een specifieke service
- `svcs -L apache` : toon de locatie van het logbestand van een service
- `tail $(svcs -L apache)` : toon de laatste lijnen van het logbestand van een service

## Links
- More information: https://www.unix.com/man-page/sunos/1/svcs
