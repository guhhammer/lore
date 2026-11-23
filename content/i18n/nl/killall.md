# killall

Verstuur een kill-signaal naar alle instanties van een proces op naam (moet exact overeenkomen). Alle signalen behalve `SIGKILL` en `SIGSTOP` kunnen door het proces worden onderschept, waardoor een nette afsluiting mogelijk is.

## Usage
- `killall proces_naam` : beëindig een proces met behulp van het standaard `SIGTERM` (terminate) signaal
- `killall -l` : toon beschikbare signaalnamen (te gebruiken zonder het `SIG`-voorvoegsel)
- `killall -i proces_naam` : vraag interactief om bevestiging voordat het proces wordt beëindigd
- `killall -2 proces_naam` : beëindig een proces met het `SIGINT` (interrupt) signaal, hetzelfde signaal dat wordt verzonden door `<Ctrl c>` in te drukken
- `killall -9 proces_naam` : forceer het beëindigen van een proces

## Links
- More information: https://manned.org/killall
