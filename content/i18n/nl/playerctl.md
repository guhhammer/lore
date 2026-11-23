# playerctl

Bestuur mediaspelers via MPRIS.

## Usage
- `playerctl play-pause` : start/stop met afspelen
- `playerctl next` : ga naar het volgende nummer
- `playerctl previous` : ga terug naar het vorige nummer
- `playerctl -l` : toon alle spelers
- `playerctl -p spelernaam play-pause|next|previous|...` : stuur een commando naar een specifieke speler
- `playerctl -a play-pause|next|previous|...` : stuur een commando naar alle spelers
- `playerctl metadata -f "Huidig nummer: \{\{artist\}\} - \{\{album\}\} - \{\{title\}\}"` : toon metadata over het huidige nummer

## Links
- More information: https://github.com/altdesktop/playerctl#using-the-cli
