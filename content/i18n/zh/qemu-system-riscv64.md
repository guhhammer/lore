# qemu-system-riscv64

模拟 `riscv64` 架构。

## Usage
- `qemu-system-riscv64 -M virt -bios none -kernel kernel.elf -nographic` : 启动一个模拟 `riscv64` 架构的内核
- `qemu-system-riscv64 -M help` : 列出支持的机器类型
- `<Ctrl a><x>` : 退出无图形界面的 QEMU

## Links
- More information: https://www.qemu.org/docs/master/system/target-riscv.html
