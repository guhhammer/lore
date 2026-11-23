# pio test

Voer lokale testen uit op een PlatformIO project.

## Usage
- `pio test` : voer alle testen in alle omgevingen uit van het huidige PlatformIO project
- `pio test -e omgeving1 -e omgeving2` : test alleen op specifieke omgevingen
- `pio test -f "patroon"` : voer alleen testen uit die qua naam overeenkomen met een specifiek `glob`-patroon
- `pio test -i "patroon"` : negeer testen die qua naam overeenkomen met een specifiek `glob`-patroon
- `pio test --upload-port upload_poort` : specificeer een poort voor het uploaden van firmware
- `pio test -c pad/naar/platformio.ini` : specificeer een aangepast configuratiebestand voor het uitvoeren van de testen

## Links
- More information: https://docs.platformio.org/en/latest/core/userguide/cmd_test.html
