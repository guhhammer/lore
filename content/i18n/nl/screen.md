# screen

Houd een sessie open op een externe server. Beheer meerdere vensters met één SSH-verbinding.

## Usage
- `screen` : start een nieuwe screen-sessie
- `screen -S sessie_naam` : start een nieuwe benoemde screen-sessie
- `screen -dmLS sessie_naam commando` : start een nieuwe daemon en log de output naar `screenlog.x`
- `screen -ls` : toon open screen-sessies
- `screen -r sessie_naam` : herkoppel aan een open screen
- `<Ctrl a><d>` : koppel los van binnen een screen
- `<Ctrl a><k>` : beëindig de huidige screen-sessie
- `screen -X -S sessie_naam quit` : beëindig een losgekoppelde screen

## Links
- More information: https://manned.org/screen
