# tmux

Terminal multiplexer. Het maakt meerdere sessies met vensters, panes en meer mogelijk.

## Usage
- `tmux` : start een nieuwe sessie
- `tmux new -s naam` : start een nieuwe benoemde sessie
- `tmux ls` : toon bestaande sessies
- `tmux a -t naam` : koppel aan een benoemde sessie (het target kan worden weggelaten om aan de meest recent gebruikte sessie te koppelen)
- `<Ctrl b><d>` : koppel los van de huidige sessie (binnen een tmux-sessie)
- `<Ctrl b><c>` : creëer een nieuw venster (binnen een tmux-sessie)
- `<Ctrl b><w>` : wissel tussen sessies en vensters (binnen een tmux-sessie)
- `tmux kill-session -t naam` : sluit een sessie op basis van de doelnaam (t)

## Links
- More information: https://github.com/tmux/tmux
