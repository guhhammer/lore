# npm query

Print een array van afhankelijkheidsobjecten met behulp van CSS-achtige selectors.

## Usage
- `npm query ':root > *'` : print directe afhankelijkheden
- `npm query ':root > .prod|dev'` : print alle directe productie-/ontwikkelingsafhankelijkheden
- `npm query '#pakket'` : print afhankelijkheden met een specifieke naam
- `npm query '#pakket@semantische_versie'` : print afhankelijkheden met een specifieke naam en binnen een semantisch versiebereik
- `npm query ':empty'` : print afhankelijkheden die geen afhankelijkheden hebben
- `npm query ":attr(scripts, [postinstall])" | jq 'map(.name) | join("\n")' -r | xargs -I _ npm uninstall _` : zoek alle afhankelijkheden met postinstall-scripts en verwijder ze
- `npm query ":type(git)" | jq 'map(.name)' | xargs -I _ npm why _` : zoek alle Git-afhankelijkheden en print welke applicatie ze vereist

## Links
- More information: https://docs.npmjs.com/cli/npm-query/
