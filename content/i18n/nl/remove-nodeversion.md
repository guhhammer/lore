# remove-nodeversion

Deïnstalleer Node.js runtime versies voor `ps-nvm`. Onderdeel van `ps-nvm` en kan alleen uitgevoerd worden in PowerShell.

## Usage
- `Remove-NodeVersion node_versie` : deïnstalleer een gegeven Node.js versie
- `Remove-NodeVersion node_versie1 , node_versie2 , ...` : deïnstalleer meerdere Node.js versies
- `Get-NodeVersions -Filter ">=20.0.0 <21.0.0" | Remove-NodeVersion` : deïnstalleer alle huidige geïnstalleerde versies van Node.js 20.x
- `Get-NodeVersions | Remove-NodeVersion` : deïnstalleer alle huidige geïnstalleerde versies van Node.js

## Links
- More information: https://github.com/aaronpowell/ps-nvm
