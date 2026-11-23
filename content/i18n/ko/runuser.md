# runuser

사용자와 그룹으로 명령을 비밀번호 없이 실행 (루트 권한 필요).

## Usage
- `sudo runuser 사용자 -c '명령'` : 다른 사용자로 명령 실행
- `sudo runuser 사용자 -g 그룹 -c '명령'` : 다른 사용자 및 그룹으로 명령 실행
- `sudo runuser 사용자 -l` : 특정 사용자로 로그인 셸 시작
- `sudo runuser 사용자 -s /bin/sh` : 기본 셸 대신 특정 셸을 지정하여 실행 (로그인에도 작동)
- `sudo runuser 사용자 -p -c '명령'` : 루트의 전체 환경을 보존 (단, `--login`이 지정되지 않은 경우에만)

## Links
- More information: https://manned.org/runuser
