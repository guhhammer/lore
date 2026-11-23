# amdgpu_top

Tool to display AMD GPU utilization and hardware metrics using the AMDGPU driver.

## Usage
- `amdgpu_top --list` : display a list of AMDGPU devices
- `amdgpu_top -p` : dump all GPU processes and per-process memory usage
- `amdgpu_top --pci "0000:01:00.0"` : select a specific GPU by PCI bus
- `amdgpu_top` : launch the interactive TUI monitor
- `amdgpu_top --gui` : launch the GUI monitor
- `amdgpu_top --smi` : launch a simple SMI-like TUI view

## See also
- nvtop
- radeontop

## Categories
hardware, monitoring, desktop

## Links
- More information: https://github.com/Umio-Yasuno/amdgpu_top#usage
- tldr page: https://tldr.inbrowser.app/pages/linux/amdgpu_top
