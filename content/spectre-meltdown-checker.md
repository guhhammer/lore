# spectre-meltdown-checker

Spectre and Meltdown mitigation detection tool.

## Usage
- `sudo spectre-meltdown-checker` : check the currently running kernel for Spectre or Meltdown
- `sudo spectre-meltdown-checker --explain` : check the currently running kernel and show an explanation of the actions to take to mitigate a vulnerability
- `sudo spectre-meltdown-checker --variant 1|2|3|3a|4|l1tf|msbds|mfbds|mlpds|mdsum|taa|mcespc|srbds` : check for specific variants (defaults to all)
- `sudo spectre-meltdown-checker --batch text|json|nrpe|prometheus|short` : display output using a specific output format
- `sudo spectre-meltdown-checker --no-sysfs` : don't use the `/sys` interface even if present
- `sudo spectre-meltdown-checker --kernel path/to/kernel_file` : check a non-running kernel

## Categories
users, system, network

## Links
- More information: https://manned.org/spectre-meltdown-checker
- tldr page: https://tldr.inbrowser.app/pages/linux/spectre-meltdown-checker
