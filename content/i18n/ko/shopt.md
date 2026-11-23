# shopt

Bash 셸 옵션 관리: Bash 셸에 특화된 동작을 제어하는 변수(`$BASHOPTS`에 저장). 일반적인 POSIX 셸 변수는 `set` 명령으로 대신 관리 (`$SHELLOPTS`에 저장).

## Usage
- `shopt` : 설정 가능한 모든 옵션과 설정 여부 나열
- `shopt -s 옵션_이름` : 옵션 설정
- `shopt -u 옵션_이름` : 옵션 해제
- `shopt -p` : 실행 가능한 `shopt` 명령으로 형식화된 모든 옵션과 상태 목록 출력
- `help shopt` : 도움말 표시

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#The-Shopt-Builtin
