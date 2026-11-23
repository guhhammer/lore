# networkctl

네트워크 링크의 상태를 조회합니다. `systemd-networkd`를 사용하여 네트워크 구성을 관리합니다.

## Usage
- `networkctl` : 기존 링크와 그 상태를 나열
- `networkctl status` : 전체 네트워크 상태 표시
- `sudo networkctl up 인터페이스1 인터페이스2 ...` : 네트워크 장치를 활성화
- `sudo networkctl down 인터페이스1 인터페이스2 ...` : 네트워크 장치를 비활성화
- `sudo networkctl renew 인터페이스1 인터페이스2 ...` : 동적 구성 갱신 (예: DHCP 서버로부터 받은 IP 주소)
- `sudo networkctl reload` : 구성 파일(`.netdev` 및 `.network`) 재로드
- `sudo networkctl reconfigure 인터페이스1 인터페이스2 ...` : 네트워크 인터페이스 재구성 (구성을 편집한 경우, 먼저 `sudo networkctl reload`를 호출해야 함)

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/networkctl.html
