# chgrp

Verander de eigenaarsgroep van bestanden en mappen.

## Usage
- `chgrp groep pad/naar/bestand_of_map` : verander de eigenaarsgroep van een bestand of map
- `chgrp -R groep pad/naar/map` : verander recursief de eigenaarsgroep van een map en alle bestanden erin
- `chgrp -h groep pad/naar/symlink` : verander de eigenaarsgroep van een symbolische link
- `chgrp --reference pad/naar/referentiebestand pad/naar/bestand_of_map` : verander de eigenaarsgroep van een bestand/map naar dezelfde als een referentiebestand
