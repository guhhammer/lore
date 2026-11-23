# dirsearch

웹 경로 스캐너.

## Usage
- `dirsearch -u url --extensions-list` : 공통 확장자를 가진 공통 경로를 웹 서버에서 검색
- `dirsearch -l 경로/대상/url-list.txt -e php` : `.php` 확장자를 사용하여 일반 경로에 대한 웹 서버 목록을 스캔
- `dirsearch -u url --extensions-list -w 경로/대상/url-paths.txt` : 공통 확장자를 사용하여 사용자 정의 경로를 웹 서버에서 검색
- `dirsearch -u url -e php --cookie 쿠키` : 쿠키를 사용하여 웹 서버를 검색
- `dirsearch -u url -e php -m HEAD` : `HEAD` HTTP 메소드를 사용하여 웹 서버를 스캔
- `dirsearch -u url -e php --json-report 경로/대상/리포트.json` : 웹 서버를 스캔하고, 결과를 `.json` 파일에 저장

## Links
- More information: https://github.com/maurosoria/dirsearch#options
