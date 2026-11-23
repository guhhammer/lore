# tookie-osint

사용자 이름 기반 공개 정보를 수집하는 OSINT 스캐너 .

## Usage
- `tookie-osint -u 사용자명` : 사용자 이름 검색
- `tookie-osint -u 사용자명 -o json -t 10` : JSON 형식으로 출력하고 10개의 스레드를 사용하여 사용자 이름 검색
- `tookie-osint -U path/to/users.txt -o csv` : 파일에 있는 여러 사용자 이름을 검색하고 CSV 형식으로 출력
- `tookie-osint -u 사용자명 -p http://127.0.0.1:8080 -a` : 프록시를 사용하여 사용자 이름을 검색하고 모든 결과 표시
- `tookie-osint -u 사용자명 -W` : 웹 스크래퍼를 활성화하여 사용자 이름 검색
- `tookie-osint -h` : 도움말 표시

## Links
- More information: https://github.com/Alfredredbird/tookie-osint
