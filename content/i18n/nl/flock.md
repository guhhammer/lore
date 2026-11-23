# flock

Beheer bestandslocks van shell-scripts. Het kan gebruikt worden om ervoor te zorgen dat slechts één instantie van een commando draait.

## Usage
- `flock pad/naar/lock.lock commando` : voer een commando met een bestandslock uit zodra de lock beschikbaar is
- `flock -n pad/naar/lock.lock commando` : voer een opdracht uit met een bestandslock, of sluit het programma af als de lock momenteel wordt vastgehouden (met foutcode 1)
- `flock -n -E 123 pad/naar/lock.lock commando` : voer een opdracht uit met een bestandslock, of sluit af met een specifieke foutcode als de lock momenteel wordt vastgehouden
- `flock -w 10 pad/naar/lock.lock commando` : voer een commando uit met een bestandslock en wacht maximaal 10 seconden tot de lock beschikbaar is voordat wordt opgegeven
- `flock pad/naar/backup.lock tar -cvf pad/naar/backup.tar pad/naar/data/` : maak een back-up van een aantal bestanden, wacht tot het vorige `tar`-commando klaar is als dit elders nog wordt uitgevoerd en dezelfde bestandslock vasthoudt (kan gebruikt worden in een `cron` job die vaak wordt uitgevoerd)

## Links
- More information: https://manned.org/flock
