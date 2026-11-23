# qemu-system-riscv64

Emula a arquitetura `riscv64`.

## Usage
- `qemu-system-riscv64 -M virt -bios none -kernel kernel.elf -nographic` : inicializa um kernel emulando a arquitetura `riscv64`
- `qemu-system-riscv64 -M help` : lista os tipos de máquina suportados
- `<Ctrl a><x>` : sai do QEMU sem gráficos

## Links
- More information: https://www.qemu.org/docs/master/system/target-riscv.html
