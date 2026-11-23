# minidlna

ReadyMedia(이전 MiniDLNA)는 DLNA/UPnP-AV 클라이언트와 호환되는 경량 미디어 서버. 스마트 TV, 게임 콘솔 및 기타 DLNA 지원 장치로 미디어 스트리밍 가능. 일반적으로 `minidlna.conf` 파일을 설정 파일로 사용해 구성.

## Usage
- `minidlna` : 기본 설정 파일 `/etc/minidlna.conf`을 사용하여 MiniDLNA 데몬 시작
- `minidlna -f 경로/대상/minidlna.conf` : 특정 설정 파일을 사용해 MiniDLNA 시작
- `minidlna -R` : 시작할 때 미디어 데이터베이스 강제 재스캔
- `minidlna -d` : 포그라운드 모드로 MiniDLNA 실행 (디버깅에 유용)
- `minidlna -d -v` : 상세 디버그 로그 출력
- `minidlna -m 경로/대상/media` : 사용자 지정 미디어 디렉터리 지정 (설정 파일의 값을 덮어 씀)
- `minidlna -P 경로/대상/pidfile` : 사용자 지정 PID 파일 위치 지정
- `minidlna -l 경로/대상/logfil.log` : 사용자 지정 로그 파일 위치 지정

## Links
- More information: https://manned.org/minidlna
