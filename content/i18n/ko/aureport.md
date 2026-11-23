# aureport

auditd 로그를 요약한 리포트를 생성하는 도구.

## Usage
- `sudo aureport` : auditd 이벤트 요약 출력
- `sudo aureport -l` : 로그인 이벤트 요약 생성
- `sudo aureport -s` : 모든 시스템 호출(syscall) 보고서 출력
- `sudo aureport -x` : 실행 파일 이벤트 요약 생성
- `sudo aureport -ts 시작_시간 -te 종료_시간` : 특정 시간 범위의 이벤트 요약 출력
- `sudo aureport -t` : 모든 audit 로그 파일과 각 파일의 시간 범위 출력
- `aureport --help` : 도움말 표시

## Links
- More information: https://manned.org/aureport
