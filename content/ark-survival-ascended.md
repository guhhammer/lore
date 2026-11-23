# ark-survival-ascended

Create and start a headless ARK: Survival Ascended server.

## Usage
- `path/to/ArkAscendedServer TheIsland_WP` : start the server with a specific map
- `path/to/ArkAscendedServer TheIsland_WP?SessionName=session_name?ServerPassword=server_password?ServerAdminPassword=admin_password` : start the server with a specific session name, server password, and admin password
- `path/to/ArkAscendedServer TheIsland_WP -port=7777 -WinLiveMaxPlayers=1..70` : start the server with a specific port and set a maximum player count
- `path/to/ArkAscendedServer TheIsland_WP?ServerPVE=true` : enable PvE and disabling PvP
- `path/to/ArkAscendedServer TheIsland_WP?DifficultyOffset=1.0` : set a multiplier to scale the server difficulty, affecting the maximum level of wild creatures
- `path/to/ArkAscendedServer TheIsland_WP -AlwaysTickDedicatedSkeletalMeshes` : disable creature animation optimization to prevent collision issues
- `path/to/ArkAscendedServer TheIsland_WP -mods=mod_id1,mod_id2,...` : enable specific mods by their ID (comma-separated)
- `path/to/ArkAscendedServer TheIsland_WP -ServerPlatform=PC+XSX+PS5` : allow connections from specific platforms

## Categories
files, network

## Links
- More information: https://ark.wiki.gg/wiki/Server_configuration
- tldr page: https://tldr.inbrowser.app/pages/linux/ark-survival-ascended
