# virsh pool-list

가상 머신 스토리지 풀에 대한 정보 나열.

## Usage
- `virsh pool-list` : 이름, 상태 및 활성 스토리지 풀에 대한 자동 시작의 활성화 또는 비활성화 여부를 나열
- `virsh pool-list --all|inactive` : 활성 및 비활성 또는 비활성 스토리지 풀에 대한 정보 나열
- `virsh pool-list --details` : 활성 스토리지 풀의 지속성, 용량, 할당 및 사용 가능한 공간에 대한 확장 정보 나열
- `virsh pool-list --autostart|no-autostart` : 자동 시작이 활성화되거나 비활성화된 활성 스토리지 풀에 대한 정보 나열
- `virsh pool-list --persistent|transient` : 지속적이거나 일시적인 활성 스토리지 풀에 대한 정보 나열
- `virsh pool-list --name --uuid` : 활성 스토리지 풀의 이름 및 UUID 나열

## Links
- More information: https://manned.org/virsh
