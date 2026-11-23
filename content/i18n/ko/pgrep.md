# pgrep

이름으로 프로세스 검색.

## Usage
- `pgrep 프로세스_이름` : 실행 중인 프로세스 중 명령 문자열이 일치하는 PID 반환
- `pgrep -l 프로세스_이름` : PID와 함께 프로세스 이름 출력 (긴(long) 출력 모드)
- `pgrep -f "프로세스_이름 인자"` : 프로세스 이름만이 아니라 전체 인자(full argument) 목록 기준으로 매칭
- `pgrep -fl "프로세스_이름"` : PID와 전체 인자 목록(full argument list)을 함께 출력
- `pgrep -u 사용자명 프로세스_이름` : 특정 사용자(user) (effective UID)가 실행한 프로세스 검색
- `pgrep -n 프로세스_이름` : 가장 최근에(newest) 시작된 일치 프로세스만 선택

## Links
- More information: https://keith.github.io/xcode-man-pages/pgrep.1.html
