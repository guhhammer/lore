# setarch

Change the reported architecture for a program's execution, primarily used to modify how programs behave based on system architecture. Useful for compatibility testing or running legacy applications.

## Usage
- `setarch i686 command` : run a command as if the machine architecture is `i686` (useful for running 32-bit apps on a 64-bit kernel)
- `setarch x86_64 bash` : run a shell with the `x86_64` architecture
- `setarch linux32 -R command` : disable randomization of the virtual address space
- `setarch --list` : list supported architectures
- `setarch -h` : display help

## Links
- More information: https://manned.org/setarch
- tldr page: https://tldr.inbrowser.app/pages/linux/setarch
