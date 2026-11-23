# npm

JavaScript en Node.js pakketbeheerder. Beheer Node.js-projecten en hun module-afhankelijkheden. Sommige subcommando's zoals `install`, `run`, etc. hebben hun eigen documentatie.

## Usage
- `npm init -y` : maak een `package.json`-bestand met standaardwaarden (laat `--yes` weg om dit interactief te doen)
- `npm i` : download alle pakketten die zijn vermeld als afhankelijkheden in `package.json`
- `npm i pakket_naam@versie` : download een specifieke versie van een pakket en voeg het toe aan de lijst van afhankelijkheden in `package.json`
- `npm i pakket_naam -D` : download de nieuwste versie van een pakket en voeg het toe aan de lijst van dev-afhankelijkheden in `package.json`
- `npm i pakket_naam -g` : download de nieuwste versie van een pakket en installeer het globaal (stel de installatielocatie in met `npm config set prefix`)
- `npm r pakket_naam` : verwijder een pakket en haal het uit de lijst van afhankelijkheden in `package.json`
- `npm ls` : toon alle lokaal geïnstalleerde afhankelijkheden
- `npm ls -g --depth 0` : toon alle top-level globaal geïnstalleerde pakketten

## Links
- More information: https://docs.npmjs.com/cli/npm/
