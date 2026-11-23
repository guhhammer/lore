# nms

1992년 영화 Sneakers에서 볼 수 있는 유명한 데이터 해독 효과를 `stdin`에서 재현하는 명령줄 도구.

## Usage
- `echo "Hello, World!" | nms` : 키 입력 후 텍스트 해독
- `ls -la | nms -a` : 키 입력을 기다리지 않고 즉시 출력 해독
- `cat 경로/대상/파일 | nms -a -f blue|white|yellow|black|magenta|green|red` : 파일의 내용을 해독하고, 사용자 지정 출력 색상 사용
- `명령어 | nms -a -c` : 해독하기 전에 화면 지우기

## Links
- More information: https://github.com/bartobri/no-more-secrets
