# pio remote

Helper command for PlatformIO Remote Development. `pio remote [command]` takes the same arguments as its locally executing counterpart `pio [command]`.

## Usage
- `pio remote agent list` : list all active Remote Agents
- `pio remote agent start -n agent_name -s example1@example.com -s example2@example.com` : start a new Remote Agent with a specific name and share it with friends
- `pio remote --agent agent_name1 --agent agent_name2 device list` : list devices from specified Agents (omit `--agent` to specify all Agents)
- `pio remote --agent agent_name device monitor` : connect to the serial port of a remote device
- `pio remote --agent agent_name run` : run all targets on a specified Agent
- `pio remote --agent agent_name update` : update installed core packages, development platforms, and global libraries on a specific Agent
- `pio remote --agent agent_name test` : run all tests in all environments on a specific Agent

## Categories
hardware, network

## Links
- More information: https://docs.platformio.org/en/latest/core/userguide/remote/index.html
- tldr page: https://tldr.inbrowser.app/pages/common/pio-remote
