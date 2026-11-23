# journalctl

Bevraag het systemd-journaal.

## Usage
- `journalctl -n n -f` : toon de laatste `n` regels en volg nieuwe berichten (zoals `tail --follow` voor traditionele syslog)
- `journalctl -b -1 -p 3` : toon alle berichten met prioriteitsniveau 3 (fouten) van de opstart voorafgaand aan de laatste afsluiting
- `journalctl -u unit` : toon alle berichten van een specifieke unit
- `journalctl _SYSTEMD_INVOCATION_ID=$(systemctl show --value --property=InvocationID unit)` : toon logs voor een gegeven unit sinds de laatste keer dat deze is gestart
- `journalctl -S now|today|yesterday|tomorrow|... -U "YYYY-MM-DD HH:MM:SS"` : filter berichten binnen een tijdsbereik (ofwel een tijdstempel ofwel plaatshouders zoals "yesterday")
- `journalctl _PID=proces_id` : toon alle berichten van een specifiek proces
- `journalctl pad/naar/uitvoerbaar_bestand` : toon alle berichten van een specifiek uitvoerbaar bestand
- `journalctl --vacuum-time 2d` : verwijder journaal-logs die ouder zijn dan 2 dagen

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/journalctl.html
