# btm

Display system information about the CPU, memory, disks, network, and processes. An enhanced alternative to `top`.

## Usage
- `btm` : show the default layout (CPU, memory, temperatures, disk, network, and processes)
- `btm -b` : enable basic mode, removing charts and condensing data (similar to `top`)
- `btm -m` : use big dots instead of small ones in charts
- `btm --battery` : show also battery charge and health status
- `btm -r 250 -t 30000` : refresh every 250 milliseconds and show the last 30 seconds in the charts

## See also
- btop
- glances
- atop
- htop
- top
- sensors

## Categories
hardware, network, monitoring

## Links
- More information: https://clementtsang.github.io/bottom/nightly/#usage-and-configuration
- tldr page: https://tldr.inbrowser.app/pages/common/btm
