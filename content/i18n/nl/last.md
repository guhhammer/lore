# last

Toon de laatst ingelogde gebruikers.

## Usage
- `last` : bekijk de laatste inloggegevens (bijv. gebruikersnaam, terminal, opstarttijd, kernel) van alle gebruikers zoals gelezen uit `/var/log/wtmp`
- `last gebruikersnaam` : toon de inloggegevens van een specifieke gebruiker
- `last -n aantal_aanmeldingen` : specificeer hoeveel van de laatste aanmeldingen weergegeven moeten worden
- `last -F -a` : toon de volledige datum en tijd voor vermeldingen en toon vervolgens de kolom met de hostnaam als laatste weer om afkapping te voorkomen
- `last gebruikersnaam -i` : toon alle aanmeldingen van een specifieke gebruiker en toon het IP-adres in plaats van de hostnaam
- `last -s -7days` : toon informatie vanaf een specifieke datum en tijd
- `last reboot` : bekijk alle geregistreerde herstarts (d.w.z. de laatste aanmeldingen van de pseudo-gebruiker "reboot")
- `last -h` : toon de help

## Links
- More information: https://manned.org/last
