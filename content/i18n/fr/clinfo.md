# clinfo

Afficher les plateformes et les périphériques OpenCL.

## Usage
- `clinfo` : affiche les informations sur toutes les plateformes et tous les périphériques OpenCL
- `clinfo -l` : énumère les plateformes et les appareils par nom
- `clinfo -d index_platforme:index_appareil` : affiche les informations relatives à un appareil spécifique
- `clinfo --raw` : affiche une sortie adaptée aux machines
- `clinfo --offline` : inclut les appareils hors ligne
- `clinfo -a` : tente de récupérer toutes les propriétés, y compris celles qui ne sont pas officielles
- `clinfo --prop nom_propriété` : affiche uniquement les propriétés correspondant au nom de propriété indiqué

## Links
- More information: https://manned.org/clinfo
