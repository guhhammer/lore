# apachectl

Apache HTTP 서버를 제어하는 도구.

## Usage
- `sudo apachectl start` : 서버 시작
- `sudo apachectl restart` : 서버 재시작
- `sudo apachectl stop` : 서버 중지
- `apachectl configtest` : 설정 파일 유효성 검사
- `apachectl status` : 서버 상태 확인 (lynx 브라우저 필요)
- `sudo apachectl graceful` : 연결을 끊지 않고 설정 재로드
- `apachectl -S` : 전체 Apache 설정 출력 (항상 지원되지는 않음)
- `apachectl -h` : 도움말 표시

## Links
- More information: https://manned.org/apachectl
