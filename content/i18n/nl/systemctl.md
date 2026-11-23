# systemctl

Beheer systemd, de systeem- en servicemanager. Sommige subcommando's zoals `disable`, `status`, `reboot` etc. hebben hun eigen documentatie.

## Usage
- `systemctl status` : toon alle actieve services
- `systemctl --failed` : toon gefaalde eenheden
- `systemctl start|stop|restart|reload|status eenheid` : start/Stop/Herstart/Herlaad/Toon de status van een service
- `systemctl enable|disable eenheid` : schakel een eenheid die bij het opstarten wordt uitgevoerd in/uit
- `systemctl daemon-reload` : herlaad systemd, scan voor nieuwe of veranderde eenheden
- `systemctl is-active|is-enabled|is-failed eenheid` : controleer of een eenheid actief/ingeschakeld/gefaald is
- `systemctl list-units -t service|socket|automount|... --state failed|running` : toon alle service/socket/automount eenheden, waarbij wordt gefilterd op de actief/gefaald status
- `systemctl cat|edit eenheid` : toon of bewerk de inhoud en het absolute pad van een eenheidsbestand

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/systemctl.html
