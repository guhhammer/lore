# orb

Interface for OrbStack, a fast and lightweight container and virtual machine runtime for macOS. Provides Docker-compatible commands and Linux VM management.

## Usage
- `orb list` : list all containers and VMs
- `orb create vm_name` : create and start a Linux virtual machine
- `orb create vm_name ubuntu|fedora|arch|debian` : create a VM with a specific Linux distribution
- `orb start|stop vm_name` : start or stop a virtual machine
- `orb ssh vm_name` : connect to a VM via SSH
- `orb exec vm_name command` : execute a command in a VM
- `orb delete vm_name` : delete a virtual machine
- `orb status` : show system status and resource usage

## Categories
containers, network, macos

## Links
- More information: https://docs.orbstack.dev/headless
- tldr page: https://tldr.inbrowser.app/pages/osx/orb
- Wikipedia: https://en.wikipedia.org/wiki/Orb_(software)
