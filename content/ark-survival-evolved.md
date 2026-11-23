# ark-survival-evolved

Create and start a headless ARK: Survival Evolved server.

## Usage
- `path/to/ShooterGameServer TheIsland` : start the server with a specific map
- `path/to/ShooterGameServer TheIsland?SessionName=session_name?ServerPassword=server_password?ServerAdminPassword=admin_password` : start the server with a specific session name, server password, and admin password
- `path/to/ShooterGameServer TheIsland?Port=7777?MaxPlayers=1..70` : start the server with a specific port and set a maximum player count
- `path/to/ShooterGameServer TheIsland?ServerPVE=true` : enable PvE and disabling PvP
- `path/to/ShooterGameServer TheIsland?DifficultyOffset=1.0` : set a multiplier to scale the server difficulty, affecting the maximum level of wild creatures
- `path/to/ShooterGameServer TheIsland -ActiveEvent=Summer` : enable a specific event
- `path/to/ShooterGameServer TheIsland -automanagedmods` : enable automatic mod downloading, installation, and updating (Steam only)
- `path/to/ShooterGameServer TheIsland -crossplay -PublicIPForEpic=ip_address` : enable crossplay between Steam and Epic Games Store

## Categories
files, network

## Links
- More information: https://ark.wiki.gg/wiki/Server_configuration
- tldr page: https://tldr.inbrowser.app/pages/linux/ark-survival-evolved
