# frpc

Maak verbinding met een `frps`-server om verbindingen op de huidige host te proxyen. Onderdeel van `frp`.

## Usage
- `frpc` : start de service met het standaardconfiguratiebestand (aangenomen wordt dat dit `frps.ini` is in de huidige map)
- `frpc -c pad/naar/frpc.toml` : start de service met het nieuwere TOML-configuratiebestand
- `frpc -c pad/naar/bestand` : start de service met een specifiek configuratiebestand
- `frpc verify -c pad/naar/bestand` : controleer of het configuratiebestand geldig is
- `frpc completion bash|fish|powershell|zsh` : toon het script om autocompletion op te zetten voor Bash, fish, PowerShell of Zsh
- `frpc -v` : toon de versie

## Links
- More information: https://github.com/fatedier/frp
