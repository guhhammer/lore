# nixos-container

Startet NixOS Container basierend auf Linux Containern.

## Usage
- `sudo nixos-container list` : gibt eine Liste der gestarteten Container aus
- `sudo nixos-container create container_name --config-file path/to/nix_config_file` : erstelle einen NixOS Container mit einer spezifischen Konfigurations-Datei
- `sudo nixos-container start|stop|terminate|destroy|status container_name` : starte, stoppe, terminiere oder zerstöre den angegebenen Container
- `sudo nixos-container run container_name -- command command_arguments` : führe ein Kommando in einem laufenden Container aus
- `sudo $EDITOR /var/lib/container/container_name/etc/nixos/configuration.nix && sudo nixos-container update container_name` : aktualisiere eine Containerkonfiguration
- `sudo nixos-container root-login container_name` : starte eine interaktive Shell innerhalb eines laufenden Containers

## Links
- More information: https://nixos.org/manual/nixos/stable/#ch-containers
