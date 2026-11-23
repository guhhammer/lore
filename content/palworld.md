# palworld

Create and start a headless Palworld server.

## Usage
- `path/to/PalServer.sh` : start the server with default settings
- `path/to/PalServer.sh -port=8211 -players=1..32` : start the server with a specific port and set a maximum player count
- `path/to/PalServer.sh -publiclobby` : start a public lobby server
- `path/to/PalServer.sh -useperfthreads -NoAsyncLoadingThread -UseMultithreadForDS` : start the server with performance optimizations for multi-threaded CPUs
- `path/to/PalServer.sh -publicip=ip_address -publicport=port` : start the server with a specific public IP and port for community servers

## Categories
network, files

## Links
- More information: https://docs.palworldgame.com/settings-and-operation/arguments/
- tldr page: https://tldr.inbrowser.app/pages/linux/palworld
