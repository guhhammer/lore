# piactl

상업용 VPN 제공업체인 Private Internet Access의 명령줄 도구.

## Usage
- `piactl login 경로/대상/로그인_파일` : private Internet Access에 로그인
- `piactl connect` : private Internet Access에 연결
- `piactl disconnect` : private Internet Access에서 연결 해제
- `piactl background enable|disable` : 백그라운드에서 Private Internet Access 데몬 활성화 또는 비활성화
- `piactl get regions` : 사용 가능한 모든 VPN 지역 나열
- `piactl get region` : 현재 VPN 지역 표시
- `piactl set region 지역` : VPN 지역 설정
- `piactl logout` : private Internet Access에서 로그아웃
