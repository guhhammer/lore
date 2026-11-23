# git instaweb

GitWeb 서버를 실행하는 도우미 도구.

## Usage
- `git instaweb --start` : 현재 Git 저장소에 대해 GitWeb 서버 실행
- `git instaweb --start -l` : 로컬호스트에서만 리슨
- `git instaweb --start -p 1234` : 특정 포트에서 리슨
- `git instaweb --start -d lighttpd|apache2|mongoose|plackup|webrick` : 지정된 HTTP 데몬 사용
- `git instaweb --start -b` : 웹 브라우저도 자동으로 실행
- `git instaweb --stop` : 현재 실행 중인 GitWeb 서버 중지
- `git instaweb --restart` : 현재 실행 중인 GitWeb 서버 재시작

## Links
- More information: https://git-scm.com/docs/git-instaweb
