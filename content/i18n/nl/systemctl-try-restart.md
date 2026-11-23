# systemctl try-restart

Herstart een of meer eenheden als ze momenteel actief zijn.

## Usage
- `systemctl try-restart eenheid` : herstart een specifieke eenheid als deze draait
- `systemctl try-restart eenheid1 eenheid2 ...` : herstart meerdere eenheden als ze draaien
- `systemctl try-restart 'patroon'` : herstart alle eenheden die overeenkomen met een patroon, als ze actief zijn
