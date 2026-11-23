# vzdump

가상 머신 및 컨테이너 백업 유틸리티.

## Usage
- `vzdump 가상_머신_id` : 스냅샷을 제외하고 기본 덤프 디렉토리(보통 `/var/lib/vz/dump/`)에 게스트 가상 머신을 덤프
- `vzdump 101 102 103` : ID가 101, 102, 103인 게스트 가상 머신 백업
- `vzdump 가상_머신_id --mode suspend|snapshot` : 특정 모드를 사용하여 게스트 가상 머신 덤프
- `vzdump --all --mode suspend --mailto root --mailto admin` : 모든 게스트 시스템을 백업하고 루트 및 관리자 사용자에게 알림 이메일 전송
- `vzdump 가상_머신_id --dumpdir 경로/대상/폴더 --mode snapshot` : 스냅샷 모드 사용(다운타임 필요 없음) 및 기본이 아닌 덤프 디렉토리 사용
- `vzdump --mode suspend --exclude 101, 102` : ID가 101 및 102인 것 제외한 모든 게스트 가상 머신 백업

## Links
- More information: https://pve.proxmox.com/pve-docs/vzdump.1.html
