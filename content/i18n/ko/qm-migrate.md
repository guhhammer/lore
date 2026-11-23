# qm migrate

가상 머신을 마이그레이션. 새로운 마이그레이션 작업을 생성하는 데 사용됩니다.

## Usage
- `qm mi 100 target` : 특정 가상 머신 마이그레이션
- `qm mi 100 target --bwlimit 10` : 현재 I/O 대역폭 제한을 10 KiB/s로 재정의
- `qm mi 100 target --force true` : 로컬 장치를 사용하는 가상 머신의 마이그레이션 허용 (루트 전용)
- `qm mi 100 target --online true` : 가상 머신이 실행 중인 경우 온라인/라이브 마이그레이션 사용
- `qm mi 100 target --with-local-disks true` : 로컬 디스크에 대한 라이브 스토리지 마이그레이션 활성화

## Links
- More information: https://pve.proxmox.com/pve-docs/qm.1.html#cli_qm_migrate
