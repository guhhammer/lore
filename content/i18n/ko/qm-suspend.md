# qm suspend

Proxmox Virtual Environment (PVE)에서 가상 머신(VM)을 일시 중단합니다. `--skiplock` 및 `--skiplockstorage` 플래그는 데이터 손상을 초래할 수 있으므로 주의해서 사용해야 합니다.

## Usage
- `qm su 100 정수` : ID로 가상 머신 일시 중단
- `qm su 100 정수 --skiplock` : VM을 일시 중단할 때 잠금 확인 건너뛰기
- `qm su 100 정수 --skiplockstorage` : VM을 일시 중단할 때 스토리지 잠금 확인 건너뛰기

## Links
- More information: https://pve.proxmox.com/pve-docs/qm.1.html#cli_qm_suspend
