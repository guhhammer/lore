# update-alternatives

Onderhoud handig symbolische links om standaard commando's te bepalen.

## Usage
- `sudo update-alternatives --install pad/naar/symlink commando_naam pad/naar/commando_binary prioriteit` : voeg een symbolische link toe
- `sudo update-alternatives --config java` : configureer een symbolische link voor `java`
- `sudo update-alternatives --remove java /opt/java/jdk1.8.0_102/bin/java` : verwijder een symbolische link
- `update-alternatives --display java` : toon informatie over een specifiek commando
- `update-alternatives --get-selections` : toon alle commando's en hun huidige selectie

## Links
- More information: https://manned.org/update-alternatives
