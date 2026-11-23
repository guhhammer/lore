# watch

명령어를 반복 실행하고 출력 결과를 전체 화면 모드로 모니터링합니다.

## Usage
- `watch ls` : 현재 디렉토리의 파일 모니터링
- `watch -d df` : 디스크 공간을 모니터링하고 변경 사항 강조 표시
- `watch -n 3 "ps aux | grep node"` : "node" 프로세스를 3초마다 새로고침하며 모니터링
- `watch -g df` : 디스크 공간을 모니터링하고 변경 시 모니터링 중지

## Links
- More information: https://manned.org/watch
