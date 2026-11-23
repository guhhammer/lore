# cpupower

Tools regarding CPU power and tuning options.

## Usage
- `sudo cpupower -c all info` : print information about all cores
- `sudo cpupower -c all frequency-set --governor powersave` : set all CPUs to a power-saving frequency governor
- `sudo cpupower -c 0 frequency-info -g | grep "analyzing\|governors"` : print CPU 0's available frequency governors
- `sudo cpupower -c 4 frequency-info -w -m` : print CPU 4's frequency from the hardware, in a human-readable format

## Categories
hardware, users, documents

## Links
- More information: https://manned.org/cpupower
- tldr page: https://tldr.inbrowser.app/pages/linux/cpupower
