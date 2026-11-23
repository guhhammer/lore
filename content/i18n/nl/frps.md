# frps

Stel snel een reverse proxy-service in. Onderdeel van `frp`.

## Usage
- `frps` : start de service met het standaardconfiguratiebestand (aangenomen wordt dat dit `frps.ini` is in de huidige map)
- `frps -c pad/naar/frps.toml` : start de service met het nieuwere TOML-configuratiebestand
- `frps -c pad/naar/bestand` : start de service met een specifiek configuratiebestand
- `frps verify -c pad/naar/bestand` : controleer of het configuratiebestand geldig is
- `frps completion bash|fish|powershell|zsh` : toon het script om autocompletion op te zetten voor Bash, fish, PowerShell of Zsh
- `frps -v` : toon de versie

## Links
- More information: https://github.com/fatedier/frp
