# radeontop

Show utilization of AMD GPUs. May require root privileges depending on your system.

## Usage
- `radeontop` : show the utilization of the default AMD GPU
- `radeontop -c` : enable colored output
- `radeontop -b bus_number` : select a specific GPU (the bus number is the first number in the output of `lspci`)
- `radeontop -t samples_per_second` : specify the display refresh rate (higher means more GPU overhead)

## See also
- nvtop
- amdgpu_top

## Categories
hardware

## Links
- More information: https://github.com/clbr/radeontop/blob/master/radeontop.asc
- tldr page: https://tldr.inbrowser.app/pages/linux/radeontop
