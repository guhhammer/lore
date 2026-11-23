# flask-unsign

`Flask` 세션 쿠키를 브루트포스, 디코딩 및 생성하는 도구.

## Usage
- `flask-unsign -d -c 쿠키` : flask 세션 쿠키 디코딩
- `flask-unsign -d --server url` : `Set-Cookie` 헤더를 반환하는 URL에서 세션 쿠키를 가져와 디코딩
- `flask-unsign -u -c 쿠키` : 기본 flask-unsign-wordlist를 사용하여 시크릿 키 브루트포스 수행 (`flask-unsign-wordlist` 필요)
- `flask-unsign -u -c cookie -w 경로/대상/wordlist.txt` : 사용자 지정 wordlist를 사용하여 시크릿 키 브루트포스 수행 (`--no-literal-eval`은 따옴표 없는 항목 처리 시 사용)
- `flask-unsign -s -c "{'logged_in': False}" -S 시크릿` : 시크릿 키를 사용하여 새로운 세션 쿠키 서명 생성
- `flask-unsign -s -c "{'logged_in': False}" -S 시크릿 -l` : 레거시 타임스탬프를 사용하여 세션 쿠키 서명 생성 (오래된 버전 호환 목적)
- `flask-unsign -u -c cookie -w 경로/대상/wordlist.txt -t 스레드 -nE` : 사용자 지정 스레드 수와 literal evaluation 비활성화 옵션으로 세션 쿠키 브루트포스 수행

## Links
- More information: https://github.com/Paradoxis/Flask-Unsign
