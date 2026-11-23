# pfctl

패킷 필터 장치를 제어.

## Usage
- `sudo pfctl -e` : 패킷 필터 활성화
- `sudo pfctl -d` : 패킷 필터 비활성화
- `sudo pfctl -f 경로/대상/pf.conf` : 설정 파일에서 규칙 불러오기
- `pfctl -sr` : 현재 활성화된 모든 규칙 표시
- `pfctl -s info` : 패킷 필터 상태 정보 표시
- `sudo pfctl -F rules` : 모든 규칙을 초기화

## Links
- More information: https://man.freebsd.org/cgi/man.cgi?query=pfctl
