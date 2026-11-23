# install-nodeversion

Installeer Node.js runtime versies voor `ps-nvm`. Onderdeel van `ps-nvm` en kan alleen uitgevoerd worden in PowerShell.

## Usage
- `Install-NodeVersion node_versie` : installeer een specifieke Node.js versie
- `Install-NodeVersion node_versie1 , node_versie2 , ...` : installeer meerdere Node.js versies
- `Install-NodeVersion ^20` : installeer de laatst beschikbare versie van Node.js 20
- `Install-NodeVersion node_versie -Architecture x86|x64|arm64` : installeer de x86 (x86 32-bit) / x64 (x86 64-bit) / arm64 (ARM 64-bit) versie van Node.js
- `Install-NodeVersion node_versie -Proxy http://example.com` : gebruik een HTTP proxy voor het downloaden van Node.js

## Links
- More information: https://github.com/aaronpowell/ps-nvm
