# nvm

Installeer, deïnstalleer of wissel tussen verschillende Node.js-versies. Ondersteunt versienummers zoals "12.8" of "v16.13.1", en labels zoals "node", "system", enz.

## Usage
- `nvm install|uninstall node_versie` : installeer/verwijder een specifieke versie van Node.js
- `nvm use node_versie` : gebruik een specifieke versie van Node.js in de huidige shell
- `nvm alias default node_versie` : stel de standaardversie van Node.js in
- `nvm ls-remote --lts` : toon externe versies beschikbaar voor installeren, waarbij alleen LTS (long-term support) versies worden getoond
- `nvm ls` : toon geïnstalleerde versies
- `nvm run node_versie` : start de REPL van een specifieke versie van Node.js
- `nvm exec node_versie node pad/naar/script.js` : voer een script uit in een specifieke versie van Node.js
- `nvm install-latest-npm` : upgrade naar de nieuwste werkende npm-versie op de huidige Node.js-versie

## Links
- More information: https://github.com/nvm-sh/nvm#usage
