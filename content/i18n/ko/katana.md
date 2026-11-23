# katana

자동화 파이프라인 실행에 초점을 둔 빠른 웹 크롤러 (헤드리스 및 비헤드리스 크롤링을 지원).

## Usage
- `katana -list https://example.com,https://google.com,...` : URL 목록 크롤링
- `katana -u https://example.com -hl` : chromium 기반 헤드리스 모드로 URL(uRL) 크롤링
- `katana -proxy http://127.0.0.1:8080 -H 경로/대상/헤더파일.txt -u https://example.com` : proxy (http/socks5)를 통해 요청 전달 및 사용자 지정 헤더를 사용
- `katana -s depth-first|breadth-first -d 값 -rl 값 -u https://example.com` : 크롤링 전략, 하위 디렉터리 탐색 깊이 및 요청 속도를 제한 (초당 요청)
- `subfinder -dL 경로/대상/도메인파일 | katana -ct 값 -o 경로/대상/출력파일.txt` : `subfinder`로 서브도메인을 찾고, 각 도메인을 지정시간 동안 크롤링 후, 결과를 파일로 저장

## Links
- More information: https://docs.projectdiscovery.io/opensource/katana/usage
