# cgclassify

Verplaats lopende taken naar `cgroups`.

## Usage
- `cgclassify -g cpu:student 1234` : verplaats het proces met een specifiek PID naar de controlegroep student in de CPU-hiërarchie
- `cgclassify 1234` : verplaats het proces met een specifiek PID naar de controlegroepen gebaseerd op het `/etc/cgrules.conf` configuratiebestand
- `cgclassify --sticky -g cpu:/student 1234` : verplaats het proces met een specifiek PID naar de controlegroep student in de CPU-hiërarchie. Let op: de daemon van de service `cgred` verandert `cgroups` van de specifieke PID en zijn onderliggende processen niet (gebaseerd op `/etc/cgrules.conf`)

## Links
- More information: https://manned.org/cgclassify
