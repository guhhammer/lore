# systemctl revert

Zet eenheidsbestanden terug naar hun leveranciersversies. Maakt de effecten van `edit`, `enable`, `disable`, `set-property` en `mask` ongedaan.

## Usage
- `systemctl revert eenheid1 eenheid2 ...` : zet eenheidsbestanden terug naar hun standaardinstellingen
- `systemctl revert eenheid --user` : zet een gebruikerseenheidsbestand terug
