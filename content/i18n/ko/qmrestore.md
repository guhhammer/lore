# qmrestore

QemuServer의 `vzdump` 백업 복원.

## Usage
- `qmrestore 경로/대상/vzdump-qemu-100.vma.lzo 100` : 원본 스토리지에 있는 백업 파일에서 가상 머신 복원
- `qmrestore 경로/대상/vzdump-qemu-100.vma.lzo 100 --force true` : 원본 스토리지에 있는 백업 파일에서 기존 가상 머신 덮어쓰기
- `qmrestore 경로/대상/vzdump-qemu-100.vma.lzo 100 --storage local` : 특정 스토리지에 있는 백업 파일에서 가상 머신 복원
- `qmrestore 경로/대상/vzdump-qemu-100.vma.lzo 100 --live-restore true` : 백업에서 즉시 가상 머신 시작하고 백그라운드에서 복원 진행 (Proxmox Backup Server에서만 가능)

## Links
- More information: https://pve.proxmox.com/pve-docs/qmrestore.1.html
