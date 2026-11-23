# nmcli general

NetworkManager의 일반 설정을 관리합니다.

## Usage
- `nmcli g` : networkManager의 일반 상태 표시
- `nmcli g h` : 현재 장치의 호스트명 표시
- `sudo nmcli g h 새_호스트명` : 현재 장치의 호스트명 변경
- `nmcli g p` : networkManager의 권한 표시
- `nmcli g l` : 현재 로깅 수준 및 도메인 표시
- `sudo nmcli g l l INFO|OFF|ERR|WARN|DEBUG|TRACE domain 도메인_1,도메인_2,...` : 로깅 수준 및/또는 도메인 설정 (`man NetworkManager.conf`에서 사용 가능한 모든 도메인 확인)
