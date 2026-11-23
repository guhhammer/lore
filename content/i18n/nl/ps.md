# ps

Informatie over actieve processen.

## Usage
- `ps aux` : toon alle actieve processen
- `ps auxww` : toon alle actieve processen inclusief de volledige commandoregel
- `ps aux | grep [s]tring` : zoek naar een proces dat overeenkomt met een string (de vierkante haken voorkomen dat `grep` zichzelf vindt)
- `ps -u $(id -u) -F` : toon alle processen van de huidige gebruiker in extra volledig formaat
- `ps -u $(id -u) f` : toon alle processen van de huidige gebruiker als een boomstructuur
- `ps -o ppid= -p proces_id` : verkrijg de parent PID van een proces
- `ps k size` : sorteer processen op geheugengebruik
- `ps -L -p proces_id` : toon threads van een specifiek proces

## Links
- More information: https://manned.org/ps
