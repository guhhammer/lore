# qm shutdown

QEMU/KVM 가상 머신 관리자에서 가상 머신 종료.

## Usage
- `qm shu 100` : 가상 머신 종료
- `qm shu 100 --timeout 10` : 최대 10초 대기 후 가상 머신 종료
- `qm shu 100 --keepActive true` : 저장소 볼륨을 비활성화하지 않고 가상 머신 종료
- `qm shu 100 --skiplock true` : 잠금을 건너뛰고 가상 머신 종료 (루트 사용자만 사용 가능)
- `qm shu 100 --forceStop true` : 가상 머신을 정지하고 종료

## Links
- More information: https://pve.proxmox.com/pve-docs/qm.1.html#cli_qm_shutdown
