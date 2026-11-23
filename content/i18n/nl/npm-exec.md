# npm exec

Voer binaire bestanden uit vanuit `npm`-pakketten.

## Usage
- `npm x commando argument1 argument2 ...` : voer het commando uit vanuit een lokaal of extern `npm`-pakket
- `npm x --package pakket commando` : specificeer het pakket expliciet (handig als meerdere commando's dezelfde naam hebben)
- `npm x --no-install commando argument1 argument2 ...` : voer een commando uit als het bestaat in het huidige pad of in `node_modules/.bin`
- `npm x --quiet commando argument1 argument2 ...` : voer een specifiek commando uit, waarbij uitvoer van `npm` zelf wordt onderdrukt
- `npm x --help` : toon de help

## Links
- More information: https://docs.npmjs.com/cli/npm-exec/
