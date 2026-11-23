# rexec

원격 호스트에서 명령을 실행합니다.

## Usage
- `rexec -h 원격_호스트 ls -l` : 원격 h호스트에서 명령 실행
- `rexec -u 사용자명 -h 원격_호스트 ps aux` : 원격 h호스트에서 원격 u사용자명을 지정
- `rexec -n -h 원격_호스트 ls -l` : 원격 h호스트에서 `stdin`을 `/dev/null`로 리디렉션
- `rexec -P 1234 -h 원격_호스트 ls -l` : 원격 h호스트에서 원격 P포트를 지정
