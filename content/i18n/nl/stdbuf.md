# stdbuf

Voer een commando uit met aangepaste buffering operaties voor de standaard streams.

## Usage
- `stdbuf -i 512K commando` : verander de buffergrootte van `stdin` naar 512 KiB
- `stdbuf -o L commando` : verander de buffer van `stdout` naar lijn-buffering
- `stdbuf -e 0 commando` : verander de buffer van `stderr` naar ongebufferd
