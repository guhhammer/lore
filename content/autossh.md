# autossh

Run, monitor, and restart SSH connections. Auto-reconnects to keep port forwarding tunnels up. Accepts all SSH flags.

## Usage
- `autossh -M monitor_port "ssh_command"` : start an SSH session, restarting when the Monitoring port fails to return data
- `autossh -M monitor_port -L local_port:localhost:remote_port user@host` : forward a Local port to a remote one, restarting when necessary
- `autossh -f -M monitor_port -N "ssh_command"` : fork `autossh` into the background before executing SSH and do Not open a remote shell
- `autossh -f -M 0 -N -o "ServerAliveInterval 10" -o "ServerAliveCountMax 3" "ssh_command"` : run in the background, with no monitoring port, and instead send SSH keep-alive packets every 10 seconds to detect failure
- `autossh -f -M 0 -N -o "ServerAliveInterval 10" -o "ServerAliveCountMax 3" -o ExitOnForwardFailure=yes -L local_port:localhost:remote_port user@host` : run in the background, with no monitoring port and no remote shell, exiting if the port forward fails
- `AUTOSSH_DEBUG=1 AUTOSSH_LOGFILE=path/to/autossh_log_file.log autossh -f -M monitor_port -v -E path/to/ssh_log_file.log ssh_command` : run in the background, logging `autossh` debug output and SSH verbose output to files

## Categories
network, monitoring

## Links
- More information: https://manned.org/autossh
- tldr page: https://tldr.inbrowser.app/pages/common/autossh
