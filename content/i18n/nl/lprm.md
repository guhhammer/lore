# lprm

Annuleer wachtende printtaken van een server.

## Usage
- `lprm` : annuleer de huidige taak op de standaard printer
- `lprm -h server taak_id` : annuleer een taak van een specifieke server
- `lprm -h server:poort taak_id` : annuleer een taak van een specifieke server op een specifieke poort
- `lprm -E taak_id1 taak_id2 ...` : annuleer meerdere taken met een versleutelde verbinding naar de server
- `lprm -` : annuleer alle taken
- `lprm -P bestemming/instantie` : annuleer de huidige taak van een specifieke printer of klasse

## Links
- More information: https://openprinting.github.io/cups/doc/man-lprm.html
