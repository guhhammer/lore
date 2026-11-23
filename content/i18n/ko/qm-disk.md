# qm disk

디스크 이미지 관리.

## Usage
- `qm di resi 100 디스크_이름 +nG` : 가상 디스크에 `n` 기가바이트 추가
- `qm di m 100 대상 색인` : 가상 디스크 이동
- `qm di m --delete 100 대상 색인` : 이전 가상 디스크 복사본 삭제
- `qm di i 100 경로/대상/디스크 저장소_이름 --format qcow2|raw|vmdk` : 특정 스토리지 이름을 사용하여 VMDK/`.qcow2`/raw 디스크 이미지 가져오기
- `qm di resc` : 모든 저장 장치를 다시 스캔하고 디스크 크기 및 사용하지 않는 디스크 이미지를 업데이트합니다
- `qm di resc --dryrun` : 재스캔의 드라이 런을 수행하고 구성에 대한 변경 사항을 작성하지 마십시오
- `qm di resc --vmid 100` : ID로 가상 머신을 지정합니다

## Links
- More information: https://pve.proxmox.com/pve-docs/qm.1.html#cli_qm_disk_import
