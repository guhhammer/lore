# squid

프록시 서버를 통해 HTTP 요청을 캐시하고 전달.

## Usage
- `sudo squid` : squid를 백그라운드에서 시작
- `sudo squid -N` : squid를 포그라운드에서 시작
- `sudo squid -f 경로/대상/squid.conf` : 지정한 설정 파일을 사용하여 Squid 시작
- `sudo squid -k parse` : 설정 파일에 오류가 있는지 검사
- `sudo squid -k reconfigure` : 설정 파일 다시 불러오기
- `sudo squid -k shutdown` : squid를 정상적으로 종료
- `sudo squid -k rotate` : 로그 파일 로테이션 수행

## Links
- More information: https://manned.org/squid
