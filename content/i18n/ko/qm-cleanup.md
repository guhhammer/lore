# qm cleanup

QEMU/KVM 가상 머신 관리자에서 tap 장치, VGPU 등과 같은 리소스를 정리. VM 종료, 충돌 후 호출됩니다.

## Usage
- `qm cl 100 clean-shutdown guest-requested` : 리소스 정리

## Links
- More information: https://pve.proxmox.com/pve-docs/qm.1.html#cli_qm_cleanup
