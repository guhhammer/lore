# dash

Debian Almquist Shell은 `sh`의 최신 POSIX 호환 구현 (Bash와 호환되지 않음).

## Usage
- `dash` : 대화형 쉘 세션을 시작
- `dash -c "echo 'dash가 실행중'"` : 특정 명령어(commands)를 실행
- `dash 경로/대상/스크립트.sh` : 특정 스크립트 실행
- `dash -n 경로/대상/스크립트.sh` : 구문 오류가 있는지 특정 스크립트를 확인
- `dash -x 경로/대상/스크립트.sh` : 실행하기 전에 각 명령을 출력하는 동안 특정 스크립트를 실행
- `dash -e 경로/대상/스크립트.sh` : 특정 스크립트를 실행하고 첫 번째 오류(error)에서 중지
- `echo "echo 'dash가 실행중'" | dash` : `stdin`에서 특정 명령을 실행

## Links
- More information: https://manned.org/dash
