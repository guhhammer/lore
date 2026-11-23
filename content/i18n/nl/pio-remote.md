# pio remote

Helper commando voor PlatformIO Remote Development. `pio remote [commando]` accepteert dezelfde argumenten als de lokale tegenhanger `pio [commando]`.

## Usage
- `pio remote agent list` : toon alle actieve Remote Agents
- `pio remote agent start -n agent_naam -s example1@example.com -s example2@example.com` : start een nieuwe Remote Agent met een specifieke naam en deel deze met vrienden
- `pio remote --agent agent_naam1 --agent agent_naam2 device list` : toon alle apparaten van specifieke Agents (laat `--agent` weg voor alle Agents)
- `pio remote --agent agent_naam device monitor` : verbind met een seriële poort van een remote apparaat
- `pio remote --agent agent_naam run` : voer alle doelen uit op een gespecificeerde Agent
- `pio remote --agent agent_naam update` : update geïnstalleerde kernpakketten, ontwikkelplatformen en globale bibliotheken op een specifieke Agent
- `pio remote --agent agent_naam test` : voer alle testen uit in alle omgevingen op een specifieke Agent

## Links
- More information: https://docs.platformio.org/en/latest/core/userguide/remote/index.html
