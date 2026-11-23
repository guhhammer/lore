# spotify_player

A TUI Spotify client that implements all features of the official Spotify app.

## Usage
- `spotify_player -d` : start a daemon that plays music in the background
- `spotify_player` : start the TUI (controls the daemon if available, otherwise starts its own client)
- `spotify_player -t theme_name` : use the specified theme
- `spotify_player -c path/to/directory` : use configuration files (`app.toml`, `keymap.toml`, and `theme.toml`) in the specified directory
- `spotify_player like` : like the currently playing track
- `<?>` : display a list of keybindings

## Categories
files

## Links
- More information: https://github.com/aome510/spotify-player#commands
- tldr page: https://tldr.inbrowser.app/pages/common/spotify_player
