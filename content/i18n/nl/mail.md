# mail

Werk met de mailbox van de gebruiker. Om een e-mail te versturen wordt de berichttekst opgebouwd vanaf `stdin`.

## Usage
- `mail` : open een interactieve prompt om persoonlijke mail te bekijken
- `mail --subject "onderwerpregel" to_user@example.com --cc "cc_email_adres"` : verstuur een getypt e-mailbericht met optionele CC. Het onderstaande commando gaat verder na het indrukken van `<Enter>`. Voer de berichttekst in (kan meerdere regels bevatten). Druk op `<Ctrl d>` om de berichttekst af te ronden
- `mail <pad/naar/bestand.txt --subject "$HOSTNAME bestandsnaam.txt" to_user@example.com` : verstuur een e-mail met de inhoud van een bestand
- `tar cvzf - pad/naar/map1 pad/naar/map2 | uuencode data.tar.gz | mail --subject "onderwerpregel" to_user@example.com` : verstuur een `.tar.gz` bestand als bijlage
- `mail -h` : toon de help

## Links
- More information: https://manned.org/mail
