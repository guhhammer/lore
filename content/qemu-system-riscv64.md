# qemu-system-riscv64

Emulate `riscv64` architecture.

## Usage
- `qemu-system-riscv64 -M virt -bios none -kernel kernel.elf -nographic` : boot a kernel emulating `riscv64` architecture
- `qemu-system-riscv64 -M help` : list supported machine types
- `<Ctrl a><x>` : exit non-graphical QEMU

## Categories
system

## Links
- More information: https://www.qemu.org/docs/master/system/target-riscv.html
- tldr page: https://tldr.inbrowser.app/pages/common/qemu-system-riscv64
