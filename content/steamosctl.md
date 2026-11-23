# steamosctl

Control the SteamOS system.

## Usage
- `steamosctl switch-to-desktop-mode` : change to desktop mode
- `steamosctl switch-to-game-mode` : change to gamemode
- `steamosctl set-default-login-mode game|desktop` : change whether the system opens gamemode or desktop during boot
- `steamosctl get-cpu-scheduler` : get currently used CPU scheduler
- `steamosctl set-cpu-scheduler none|lavd|...` : set the system to use a specific CPU scheduler
- `steamosctl get-valid-desktop-sessions` : display available desktop sessions
- `steamosctl set-default-desktop-session plasma.desktop|plasmax11.desktop|...` : set the default desktop session
- `steamosctl` : display help

## Categories
desktop, hardware

## Links
- More information: https://gitlab.com/evlaV/steamos-manager
- tldr page: https://tldr.inbrowser.app/pages/linux/steamosctl
