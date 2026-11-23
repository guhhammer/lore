# ps

실행 중인 프로세스에 대한 정보입니다.

## Usage
- `ps aux` : 실행 중인 모든 프로세스를 나열
- `ps auxww` : 전체 명령 문자열을 포함하여 실행 중인 모든 프로세스 나열
- `ps aux | grep 문자열` : 문자열과 일치하는 프로세스 검색
- `ps -u $(id -u) -F` : 추가 전체 형식으로 현재 사용자의 모든 프로세스를 나열
- `ps -u $(id -u) f` : 현재 사용자의 모든 프로세스를 트리로 나열
- `ps -o ppid= -p pid` : 프로세스의 상위 PID 가져오기
- `ps --sort size` : 메모리 소비를 기준으로 프로세스 정렬

## Links
- More information: https://manned.org/ps
