# find this machine's ip address

This machine's address on the local network, and the one the internet sees.

## Steps
- `ip -br addr` : every interface and its addresses (Linux)
- `hostname -I` : the addresses only (Linux)
- `ip route get 1.1.1.1` : the address used to go out: after src
- `curl -s https://ifconfig.me` : the public address the internet sees
- `ipconfig getifaddr en0` : macOS, Wi-Fi or the first network port

## See also
- ip
- hostname
- curl

## Categories
network
