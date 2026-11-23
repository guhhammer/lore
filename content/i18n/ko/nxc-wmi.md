# nxc wmi

Windows Management Instrumentation(WMI)를 통한 침투 테스트 및 익스플로잇.

## Usage
- `nxc wmi 192.168.178.2 -u 경로/대상/사용자명목록.txt -p 경로/대상/비밀번호목록.txt` : 지정된 u사용자명 및 p비밀번호 목록의 모든 조합을 시도하여 유효한 자격 증명 검색
- `nxc wmi 192.168.178.2 -u 사용자명 -p 비밀번호 --local-auth` : 로컬 인증을 통해 인증(도메인 인증이 아닌)
- `nxc wmi 192.168.178.2 -u 사용자명 -p 비밀번호 --wmi wmi_쿼리` : 지정된 WMI 쿼리 실행
- `nxc wmi 192.168.178.2 -u 사용자명 -p 비밀번호 -x 명령` : 대상 호스트에서 지정된 명령 실행

## Links
- More information: https://www.netexec.wiki/wmi-protocol/password-spraying
