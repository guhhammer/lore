# abrt-cli

Automatisch bugrapportagehulpmiddel voor Fedora-gebaseerde systemen. Gebruikt om applicatiecrashes te detecteren, analyseren en rapporteren.

## Usage
- `abrt-cli list` : toon een lijst van gedetecteerde problemen
- `abrt-cli info probleem_id` : toon details van een specifiek probleem
- `abrt-cli remove probleem_id` : verwijder een crashrapport
- `abrt-cli report probleem_id` : rapporteer een probleem aan de geconfigureerde bugtracker (bijv. Bugzilla)
- `abrt-watch-log -F error_string /var/log/myapp.log notify-send "Crash gedetecteerd"` : monitor een logbestand en start een programma als er een overeenkomst wordt gevonden
- `abrt-cli report -a probleem_id` : genereer handmatig een rapport om te debuggen

## Links
- More information: https://abrt.readthedocs.io/en/latest/usage.html
