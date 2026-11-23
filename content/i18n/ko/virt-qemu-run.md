# virt-qemu-run

`libvirtd`와 독립적으로 QEMU 게스트 VM을 실행하기 위한 실험 도구.

## Usage
- `virt-qemu-run 경로/대상/guest.xml` : QEMU 가상 머신 실행
- `virt-qemu-run -r 경로/대상/폴더 경로/대상/guest.xml` : QEMU 가상 머신을 실행하고 상태를 특정 디렉토리에 저장
- `virt-qemu-run -v 경로/대상/guest.xml` : QEMU 가상 머신을 실행하고 시작에 대한 자세한 정보 표시
- `virt-qemu-run -h` : 도움말 표시

## Links
- More information: https://libvirt.org/manpages/virt-qemu-run.html
