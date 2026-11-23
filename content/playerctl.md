# playerctl

Control media players via MPRIS.

## Usage
- `playerctl play-pause` : toggle play
- `playerctl next` : skip to the next track
- `playerctl previous` : go back to the previous track
- `playerctl -l` : list all players
- `playerctl -p player_name play-pause|next|previous|...` : send a command to a specific player
- `playerctl -a play-pause|next|previous|...` : send a command to all players
- `playerctl metadata -f "Now playing: \{\{artist\}\} - \{\{album\}\} - \{\{title\}\}"` : display metadata about the current track

## Links
- More information: https://github.com/altdesktop/playerctl#using-the-cli
- tldr page: https://tldr.inbrowser.app/pages/linux/playerctl
