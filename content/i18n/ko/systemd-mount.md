# systemd-mount

일시적인 마운트 또는 자동 마운트 포인트를 설정하고 제거.

## Usage
- `systemd-mount 경로/대상/파일_또는_장치` : 파일 시스템(이미지 또는 블록 장치)을 `/run/media/system/LABEL`에 마운트. LABEL은 파일 시스템 레이블이거나 레이블이 없는 경우 장치 이름
- `systemd-mount 경로/대상/파일_또는_장치 경로/대상/마운트_포인트` : 파일 시스템(이미지 또는 블록 장치)을 특정 위치에 마운트
- `systemd-mount --list` : 마운트할 수 있는 파일 시스템을 가진 모든 로컬, 알려진 블록 장치 나열
- `systemd-mount --automount yes 경로/대상/파일_또는_장치` : 첫 번째 접근 시 실제 파일 시스템을 마운트하는 자동 마운트 포인트 생성
- `systemd-mount -u 경로/대상/마운트_포인트_또는_장치1 경로/대상/마운트_포인트_또는_장치2 ...` : 하나 이상의 장치 언마운트
- `systemd-mount -t 파일_시스템_유형 경로/대상/파일_또는_장치 경로/대상/마운트_포인트` : 특정 파일 시스템 유형으로 파일 시스템(이미지 또는 블록 장치) 마운트
- `systemd-mount -o 마운트_옵션 경로/대상/파일_또는_장치 경로/대상/마운트_포인트` : 추가 마운트 옵션으로 파일 시스템(이미지 또는 블록 장치) 마운트

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/systemd-mount.html
