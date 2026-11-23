# qm guest exec

Execute a specific command via a guest agent.

## Usage
- `qm g exec 100 command argument1 argument2 ...` : execute a specific command via a guest agent
- `qm g exec 100 argument1 argument2 ... --synchronous 0` : execute a specific command via a guest agent asynchronously
- `qm g exec 100 argument1 argument2... --timeout 10` : execute a specific command via a guest agent with a specified timeout of 10 seconds
- `qm g exec 100 argument1 argument2 ... --pass-stdin 1` : execute a specific command via a guest agent and forward input from `stdin` until EOF to the guest agent

## Links
- More information: https://pve.proxmox.com/pve-docs/qm.1.html#cli_qm_guest_exec
- tldr page: https://tldr.inbrowser.app/pages/linux/qm-guest-exec
