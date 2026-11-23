# fsck

Controleer de integriteit van een bestandssysteem of repareer het. Het bestandssysteem moet niet gemount zijn op het moment dat het commando wordt uitgevoerd.

## Usage
- `sudo fsck /dev/sdXN` : controleer bestandssysteem `/dev/sdXN` en rapporteer beschadigde blokken
- `sudo fsck -r /dev/sdXN` : controleer bestandssysteem `/dev/sdXN`, rapporteer beschadigde blokken en laat de gebruiker interactief kiezen om elk blok te repareren
- `sudo fsck -a /dev/sdXN` : controleer bestandssysteem `/dev/sdXN`, rapporteer beschadigde blokken en repareer ze automatisch

## Links
- More information: https://manned.org/fsck
