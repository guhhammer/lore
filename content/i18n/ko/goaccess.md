# goaccess

오픈 소스 실시간 웹 로그 분석기.

## Usage
- `goaccess 경로/대상/로그파일1 경로/대상/파일2 ...` : 대화형 모드로 하나 이상의 로그 파일 분석
- `goaccess 경로/대상/로그파일 --log-format=포맷` : 특정 로그 포맷(또는 "combined" 같은 미리 정의된 포맷) 사용
- `tail -f 경로/대상/로그파일 | goaccess -` : `stdin`에서 로그 분석
- `goaccess 경로/대상/로그파일 -o 경로/대상/파일.html --real-time-html` : 로그를 실시간으로 분석하여 HTML 파일로 작성

## Links
- More information: https://goaccess.io/man
