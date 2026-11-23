# openocd

OpenOCD를 사용하여 임베디드 시스템을 디버깅 및 프로그래밍하는 도구.

## Usage
- `openocd -f 구성_파일.cfg` : 설정 파일을 사용해 보드에 OpenOCD 세션 연결
- `openocd -f config_file1.cfg -f config_file2.cfg` : 여러 설정 파일을 사용하여 보드에 OpenOCD 세션 연결
- `openocd -f config_file.cfg -c "command"` : 22설정 파일 및 서버 시작 시 실행할 명령을 함께 지정하여 보드에 OpenOCD 세션 연결
- `openocd -s path/to/search -f config_file.cfg` : 지정한 경로의 설정 파일 경로를 사용
- `target extended-remote localhost` : [대화형] OpenOCD 시작 후, GDB를 기본 포트(3333)로 연결
- `ls /usr/local/share/openocd/scripts` : 사이트 전역 스크립트 라이브러리 목록 출력

## Links
- More information: https://manned.org/openocd
