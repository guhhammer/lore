# kill

Stuur een signaal naar een proces, meestal om het proces te stoppen. Alle signalen behalve `SIGKILL` en `SIGSTOP` kunnen door het proces worden onderschept om een nette afsluiting uit te voeren.

## Usage
- `kill proces_id` : beëindig een programma met behulp van het standaard `SIGTERM` (terminate) signaal
- `kill -l` : toon beschikbare signaalnamen (te gebruiken zonder het `SIG` voorvoegsel)
- `kill -1 proces_id` : beëindig een programma met behulp van het `SIGHUP` (hang up) signaal. Veel daemons zullen herladen in plaats van beëindigen
- `kill -2 proces_id` : beëindig een programma met behulp van het `SIGINT` (interrupt) signaal. Dit wordt meestal geïnitieerd door de gebruiker die `<Ctrl c>` indrukt
- `kill -9 proces_id` : signaleer het besturingssysteem om een programma onmiddellijk te beëindigen (het programma krijgt geen kans om het signaal te onderscheppen)
- `kill -STOP proces_id` : signaleer het besturingssysteem om een programma te pauzeren totdat een `SIGCONT` ("continue") signaal wordt ontvangen
- `kill -USR1 -groep_id` : stuur een `SIGUSR1` signaal naar alle processen met de gegeven GID (groeps-ID)

## Links
- More information: https://manned.org/kill.1posix
