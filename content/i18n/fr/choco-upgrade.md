# choco upgrade

Surclasse un ou plusieurs forfaits avec Chocolatey.

## Usage
- `choco upgrade paquet1 paquet2 ...` : met à niveau un ou plusieurs paquets séparés par des espaces
- `choco upgrade paquet --version version` : met à niveau vers une version spécifique d'un paquet
- `choco upgrade all` : met à niveau tous les paquets
- `choco upgrade all --except "paquet1 paquet2 ..."` : met à niveau tous les paquets sauf ceux spécifiés, séparés par des virgules
- `choco upgrade paquet -y` : confirme automatiquement toutes les invites
- `choco upgrade paquet -s source_url|alias` : spécifie une source personnalisée à partir de laquelle recevoir les paquets
- `choco upgrade paquet -u nom_utilisateur -p mot_de_passe` : fournit un nom d'utilisateur et un mot de passe pour l'authentification

## Links
- More information: https://docs.chocolatey.org/en-us/choco/commands/upgrade/
