# mhwd-gpu

Configure graphics cards in Manjaro.

## Usage
- `mhwd-gpu --status` : show current Xorg configuration path
- `mhwd-gpu --check` : check if Xorg configuration has a valid symlink
- `sudo mhwd-gpu --setmod nvidia --setxorg /path/to/nvidia.conf` : set a custom Xorg configuration for an Nvidia GPU
- `sudo mhwd-gpu --setmod catalyst|ati --setxorg /path/to/amdgpu.conf` : set a custom Xorg configuration for an AMD GPU
- `mhwd-gpu --help` : display help

## Categories
hardware, files

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/mhwd-gpu
