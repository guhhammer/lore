# kexec

Directly reboot into a new kernel.

## Usage
- `kexec -l path/to/kernel --initrd=path/to/initrd --command-line=arguments` : load a new kernel
- `kexec -l path/to/kernel --initrd=path/to/initrd --reuse-cmdline` : load a new kernel with current boot parameters
- `kexec -e` : execute a currently loaded kernel
- `kexec -u` : unload current kexec target kernel

## Categories
system, files

## Links
- More information: https://manned.org/kexec
- tldr page: https://tldr.inbrowser.app/pages/linux/kexec
- Wikipedia: https://en.wikipedia.org/wiki/Kexec
