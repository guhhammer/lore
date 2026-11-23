# doppler run

환경에 주입된 Doppler 비밀 정보를 사용하여 명령을 실행.

## Usage
- `doppler run --command 명령어` : 명령어 실행
- `doppler run --command 명령어1 && 명령어2` : 다중 명령어 실행
- `doppler run 경로/대상/명령어.sh` : 스크립트 실행
- `doppler run -p 프로젝트_이름 -c 구성_이름 -- 명령어` : 지정된 프로젝트 및 구성으로 명령을 실행
- `doppler run --watch 명령어` : 비밀이 변경되면 자동으로 프로세스를 다시 시작
