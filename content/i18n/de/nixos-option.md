# nixos-option

Prüfe eine NixOS Konfiguration.

## Usage
- `nixos-option option_key` : liste alle Unterschlüssel eines angegebenen Options-Schlüssels
- `nixos-option boot.kernelModules` : liste aktuelle Boot-Kernelmodule
- `nixos-option users.users.username.openssh.authorizedKeys.keyFiles|keys` : liste Authorisierungsschlüssel für einen spezifischen Benutzer
- `nixos-option nix.buildMachines` : liste alle Remote-Builder-Maschinen
- `NIXOS_CONFIG=path_to_configuration.nix nixos-option option_key` : liste alle Unterschlüssel eines angegebenen Options-Schlüssels innerhalb einer angegebenen Konfigurations-Datei
- `nixos-option -r users.users.user` : zeige rekursiv alle Werte eines Benutzers

## Links
- More information: https://nixos.org/manual/nixos/stable/index.html#sec-modularity
