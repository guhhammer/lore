# virt-manager

KVM 및 Xen 가상 머신과 LXC 컨테이너를 관리하기 위한 데스크톱 사용자 인터페이스.

## Usage
- `virt-manager` : GUI 실행
- `virt-manager --connect 하이퍼바이저_uri` : 하이퍼바이저에 연결
- `virt-manager --no-fork` : 시작 시 virt-manager 프로세스를 백그라운드로 포크하지 않음
- `virt-manager --debug` : 디버그 출력 표시
- `virt-manager --show-domain-creator` : "새로운 VM" 마법사 열기
- `virt-manager --show-domain-editor 이름|id|uuid` : 특정 가상 머신/컨테이너에 대한 도메인 세부 정보 창 표시
- `virt-manager --show-domain-performance 이름|id|uuid` : 특정 가상 머신/컨테이너에 대한 도메인 성능 창 표시
- `virt-manager --show-host-summary` : 연결 세부 정보 창 표시

## Links
- More information: https://manned.org/virt-manager
