# vol.py

Forensics framework used to analyze volatile memory (RAM) dumps. With volatility3, plugins are now based on operating system. Examples below will use Windows.

## Usage
- `python3 vol.py -f path\to\memory_dump_file windows.info` : get information about a memory dump file
- `python3 vol.py -f path\to\memory_dump_file windows.pslist` : list active processes
- `python3 vol.py -f path\to\memory_dump_file windows.hashdump` : list hashes of users on system
- `python3 vol.py -f path\to\memory_dump_file windows.netstat` : list active network connections
- `python3 vol.py -h` : display help

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
files, monitoring, network, windows

## Links
- More information: https://volatility3.readthedocs.io/en/latest/index.html
- tldr page: https://tldr.inbrowser.app/pages/windows/vol.py
