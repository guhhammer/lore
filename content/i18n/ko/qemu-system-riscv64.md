# qemu-system-riscv64

`riscv64` 아키텍처를 에뮬레이션.

## Usage
- `qemu-system-riscv64 -M virt -bios none -kernel 커널.elf -nographic` : `riscv64` 아키텍처를 에뮬레이션하여 커널 부팅
- `qemu-system-riscv64 -M help` : 지원되는 머신 타입 목록 표시
- `<Ctrl a><x>` : 비그래픽 모드의 QEMU 종료

## Links
- More information: https://www.qemu.org/docs/master/system/target-riscv.html
