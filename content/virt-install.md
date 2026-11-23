# virt-install

Create virtual machines with libvirt and begin OS installation. Note: You may need to pass `--connect URI` to the commands or set up the URI in `$XDG_CONFIG_HOME/libvirt/libvirt.conf`.

## Usage
- `virt-install -n vm_name --memory 1024 --disk path=path/to/image.qcow2,size=12 -c path/to/debian.iso` : create a virtual machine with 1 GB RAM and 12 GB storage and start a Debian installation
- `virt-install -n vm_name --arch x86_64 --virt-type kvm --machine q35 --boot uefi --memory 4096 --disk path=path/to/image.raw,size=16 -c path/to/fedora.iso` : create a x86-64, KVM-accelerated, UEFI-based virtual machine with the Q35 chipset, 4 GiB RAM, 16 GiB RAW storage, and start a Fedora installation
- `virt-install -n vm_name --memory 512 --disk none --controller type=usb,model=none --sound none --autoconsole none --install no_install=yes -c path/to/tails.iso` : create a diskless live virtual machine without an emulated sound device or a USB controller. Don't start an installation and don't autoconnect to console but attach a cdrom to it (might be useful for when using a live CD like tails)
- `virt-install -n vm_name --cpu host-model,topology.sockets=1,topology.cores=4,topology.threads=2 --memory 16384 --disk path=path/to/image.qcow2,size=250 -c path/to/debian.iso` : create a virtual machine with 16 GiB RAM, 250 GiB storage, 8 cores with hyperthreading, a specific CPU topology, and a CPU model that shares most features with the host CPU
- `virt-install -n vm_name --memory 2048 --disk path=path/to/image.qcow2,size=20 -l https://download.fedoraproject.org/pub/fedora/linux/releases/43/Everything/x86_64/os/ -x "inst.ks=https://example.com/path/to/kickstart.ks"` : create a virtual machine and kickstart an automated deployment based on Fedora 43 using only remote resources (no ISO required)

## Categories
files, packages, hardware

## Links
- More information: https://manned.org/virt-install
- tldr page: https://tldr.inbrowser.app/pages/linux/virt-install
