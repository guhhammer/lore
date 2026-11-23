# steamcmd

A command-line version of the Steam client.

## Usage
- `steamcmd +login anonymous +app_update app_id +quit` : install or update an application anonymously
- `steamcmd +login username +app_update app_id +quit` : install or update an application using the specified credentials
- `steamcmd +force_install_dir path/to/directory +login anonymous +app_update app_id validate +quit` : define a custom install location
- `steamcmd +@sSteamCmdForcePlatformType windows +login anonymous +app_update app_id validate +quit` : install an application for a specific platform
- `steamcmd +runscript script.txt` : run a script file
- `steamcmd +login username +logout +quit` : clear cached login credentials for a user

## Categories
users, packages, files

## Links
- More information: https://manned.org/steamcmd
- tldr page: https://tldr.inbrowser.app/pages/common/steamcmd
