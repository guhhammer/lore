# set-nodeversion

Stel de standaard Node.js versie in voor `ps-nvm`. Onderdeel van `ps-nvm` en kan alleen uitgevoerd worden in PowerShell.

## Usage
- `Set-NodeVersion versie` : gebruik een specifieke versie van Node.js in de huidige PowerShell-sessie
- `Set-NodeVersion ^20` : gebruik de laatst geïnstalleerde Node.js versie van 20.x
- `Set-NodeVersion versie -Persist User` : stel de standaard Node.js versie in voor de huidige gebruiker (geldt alleen voor toekomstige PowerShell sessies)
- `Set-NodeVersion versie -Persist Machine` : stel de standaard Node.js versie in voor alle gebruikers (dient uitgevoerd te worden als Administrator/root en geldt alleen voor toekomstige PowerShell sessies)

## Links
- More information: https://github.com/aaronpowell/ps-nvm
