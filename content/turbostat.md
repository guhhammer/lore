# turbostat

Report processor topology, frequency, temperature, power, and idle statistics.

## Usage
- `sudo turbostat` : display statistics every 5 seconds
- `sudo turbostat -i n_seconds` : display statistics every specified amount of seconds
- `sudo turbostat --quiet` : do not decode and print the system configuration header information
- `sudo turbostat --quiet -i 1 --cpu 0-cpu_thread_count --show "PkgWatt","Busy%","Core","CoreTmp","Thermal"` : display useful information about CPU every 1 second, without header information
- `turbostat --help` : display help

## Categories
monitoring, desktop, users

## Links
- More information: https://manned.org/turbostat
- tldr page: https://tldr.inbrowser.app/pages/linux/turbostat
