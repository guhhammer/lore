# netsh

Manage Windows network settings. Some subcommands such as `wlan` have their own usage documentation.

## Usage
- `netsh add helper path\to\file.dll` : add a helper Dynamic Link Library (DLL)
- `netsh show helper` : show all loaded helper DLLs
- `netsh delete helper path\to\file.dll` : delete a helper DLL
- `netsh dump >path\to\output_file.txt` : export your network configuration settings to a file
- `netsh trace show interfaces` : show available network interfaces for tracing
- `exit` : exit the shell
- `netsh help` : display help

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
files, network, documents, windows

## Links
- tldr page: https://tldr.inbrowser.app/pages/windows/netsh
