# adb forward

Se connecte à un appareil Android sans fil.

## Usage
- `adb forward tcp:port_local tcp:port_distant` : redirige un port TCP vers le seul émulateur ou appareil connecté
- `adb -s id_appareil forward tcp:port_local tcp:port_distant` : redirige un port TCP vers un émulateur ou appareil spécifique (identifié par ID / numéro de série)
- `adb forward --list` : liste toutes les redirections
- `adb forward --remove tcp:port_local` : supprime une règle de redirection
- `adb forward --remove-all` : supprime toutes les redirections

## Links
- More information: https://developer.android.com/tools/adb
