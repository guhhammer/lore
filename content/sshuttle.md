# sshuttle

Transparent proxy server that tunnels traffic over an SSH connection. Doesn't require root or any special setup on the remote SSH server, though root access on the local machine is prompted for.

## Usage
- `sshuttle -r username@sshserver 0.0.0.0/0` : forward all IPv4 TCP traffic via a remote SSH server
- `sshuttle --dns -r username@sshserver 0.0.0.0/0` : also forward all DNS traffic to the server's default DNS resolver
- `sshuttle -r username@sshserver 0.0.0.0/0 -x 192.168.0.1/24` : forward all traffic except that which is bound for a specific subnet
- `sshuttle --method tproxy -r username@sshserver 0.0.0.0/0 ::/0 -x your_local_ip_address -x ssh_server_ip_address` : use the tproxy method to forward all IPv4 and IPv6 traffic

## Categories
network

## Links
- More information: https://manned.org/sshuttle
- tldr page: https://tldr.inbrowser.app/pages/common/sshuttle
