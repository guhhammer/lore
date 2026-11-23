# winedbg

The Wine debugger, for debugging Windows and Winelib applications.

## Usage
- `winedbg path/to/program.exe` : launch a program under the debugger
- `winedbg process_id` : attach to an already-running process by its PID
- `winedbg --gdb path/to/program.exe` : debug a program through a gdb proxy (drive it from gdb)
- `winedbg path/to/crash.mdmp` : load and inspect a minidump crash file
- `winedbg --help` : display help

## Categories
development, files, network

## Links
- More information: https://manned.org/winedbg
- tldr page: https://tldr.inbrowser.app/pages/linux/winedbg
