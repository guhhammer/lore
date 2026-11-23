# ipcs

Toon informatie over het gebruik van XSI IPC-faciliteiten: gedeelde geheugensegmenten, berichtenwachtrijen en semafoorarrays.

## Usage
- `ipcs -a` : toon informatie over alle IPC
- `ipcs -m|-q|-s` : toon informatie over actieve gedeelde memory-segmenten, berichtenwachtrijen (q) of semafoorsets
- `ipcs -b` : toon informatie over de maximaal toegestane grootte in bytes
- `ipcs -c` : toon de gebruikersnaam en groepsnaam van de creator voor alle IPC-faciliteiten
- `ipcs -p` : toon de pID van de laatste operatoren voor alle IPC-faciliteiten
- `ipcs -t` : toon toegangstijden voor alle IPC-faciliteiten
- `ipcs -o` : toon openstaand gebruik voor actieve berichtenwachtrijen en gedeelde geheugensegmenten

## Links
- More information: https://manned.org/ipcs.1p
