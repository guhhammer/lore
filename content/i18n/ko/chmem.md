# chmem

Linux 시스템에서 메모리 블록 상태(online/offline) 변경. 주로 가상화 환경에서 메모리 핫플러그(hotplug) 관리에 사용.

## Usage
- `sudo chmem -b -d 블록_번호` : 메모리 블록을 offline 상태로 설정
- `sudo chmem -b -e 블록_번호` : 메모리 블록을 online 상태로 설정
- `sudo chmem -d 0x시작_주소-0xend_address` : 16진수 주소 범위를 사용하여 메모리 영역을 offline 상태로 설정
- `sudo chmem -e 0x시작_주소-0xend_address` : 16진수 주소 범위를 사용하여 메모리 영역을 online 상태로 설정
- `sudo chmem -e 0x시작_주소 -z Movable` : 메모리를 online 상태로 설정하고 특정 존(예: Movable)에 할당
- `chmem -h` : 도움말 표시

## Links
- More information: https://manned.org/chmem
