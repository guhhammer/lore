# w32tm

Query and control the w32time time synchronization service.

## Usage
- `w32tm /query /status /verbose` : show the current status of time synchronization
- `w32tm /stripchart /computer:time_server` : show a time offset graph against a time server
- `w32tm /stripchart /packetinfo /samples:1 /computer:time_server` : show an NTP reply from a time server
- `w32tm /query /peers` : show the state of the currently used time servers
- `w32tm /query /configuration` : show configuration of the w32time service (run in elevated console)
- `w32tm /resync /force` : force time resynchronization immediately (run in elevated console)
- `w32tm /debug /enable /file:path\to\debug.log /size:10000000 /entries:0-300` : write w32time debug logs into a file (run in elevated console)

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
databases, system, development, windows

## Links
- tldr page: https://tldr.inbrowser.app/pages/windows/w32tm
