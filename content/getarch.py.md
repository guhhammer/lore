# getarch.py

Determine the OS architecture (x86 or x64) of a remote Windows system. Part of the Impacket suite.

## Usage
- `getArch.py -target target` : check the architecture of a single target system
- `getArch.py -targets path/to/targets_file` : check the architecture of multiple targets from a file (one per line)
- `getArch.py -target target -timeout seconds` : set a custom socket timeout (default is 2 seconds)
- `getArch.py -target target -debug` : enable debug mode for detailed output

## Categories
network

## Links
- More information: https://github.com/fortra/impacket
- tldr page: https://tldr.inbrowser.app/pages/common/getarch.py
