# kodi-send

Send actions to Kodi.

## Usage
- `kodi-send -a Up|Down|Left|Right|Select|Back|ContextMenu|...` : send a navigation event
- `kodi-send -a "ActivateWindow(Home|Videos|Settings|...)"` : activate a specific window
- `kodi-send -a "PlayerControl(Play|Stop|Forward|Rewind|...)"` : control the media player
- `kodi-send --button key1 --button key2 --button ...` : send keyboard key presses
- `kodi-send -a Quit` : quit Kodi
- `kodi-send -a Reboot` : reboot the system
- `kodi-send --host 192.168.0.1 --port 9777 -a Quit` : send an action to a remote host
- `kodi-send -?` : display help

## Categories
network

## Links
- More information: https://kodi.wiki/view/List_of_built-in_functions
- tldr page: https://tldr.inbrowser.app/pages/common/kodi-send
