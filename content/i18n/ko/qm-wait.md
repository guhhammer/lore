# qm wait

가상 머신이 중지될 때까지 대기.

## Usage
- `qm w 100` : 가상 머신이 중지될 때까지 대기
- `qm w 100 --timeout 10` : 가상 머신이 중지될 때까지 10초 동안 대기
- `qm shu 100 && qm w 100 --timeout 10` : 종료 요청을 전송한 후 가상 머신이 중지될 때까지 10초 동안 대기

## Links
- More information: https://pve.proxmox.com/pve-docs/qm.1.html#cli_qm_wait
