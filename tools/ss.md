# ss

ss lists network sockets: what is listening, who is connected, and which program owns each one. It replaces netstat.

## Tasks
### What is listening
- `ss -tln` : TCP ports waiting for connections, as numbers
- `ss -uln` : UDP ports in use
- `sudo ss -tlnp` : the same, with the program and PID that owns each port
- `ss -tlnp 'sport = :8080'` : who listens on port 8080
- `ss -ltn '( sport = :80 or sport = :443 )'` : check several ports at once
### Connections
- `ss -tn` : established TCP connections
- `ss -tan` : every TCP socket, any state
- `ss -tn state established` : only established ones
- `ss -tn dst 10.0.0.5` : connections to one address
- `ss -tn dport = :22` : connections to a remote port 22
- `ss -s` : a summary count by type and state
### More detail
- `ss -tnp` : show the process on each connection (root sees all)
- `ss -tni` : TCP internals: rtt, window, retransmits
- `ss -to` : show the timers (keepalive, retransmit)
- `ss -xl` : listening unix sockets
- `ss -tnH` : no header, easy to parse
- `ss -tn state time-wait | wc -l` : count sockets in TIME-WAIT
### Close a connection
- `sudo ss -K dst 10.0.0.5 dport = 22` : kill matching sockets (needs a kernel with socket destroy)

## Notes
- `-n` stops name lookups and makes ss much faster; add `-p` as root to see other users' programs.
- The flags: `t` TCP, `u` UDP, `l` listening, `a` all, `n` numeric, `p` process.
- Listening on `127.0.0.1` means only this machine can connect; `0.0.0.0` or `*` means everyone.
- A port in use by nothing here may be held by a container or another namespace.

## See also
- ip
- lsof
- netstat
- nmap

## Links
- Manual: https://man7.org/linux/man-pages/man8/ss.8.html

## Categories
network monitoring
