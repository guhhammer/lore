# kiterunner brute

워드리스트를 사용하여 API 경로 및 웹 엔드포인트를 bruteforcing하는 컨텍스트 기반 웹 스캐너. `brute` 하위 명령은 하나 이상의 호스트를 대상으로 동작합니다.

## Usage
- `kiterunner brute https://example.com -A apiroutes-210328:20000` : assetnote 워드리스트 (예: 상위 20,000개 API 경로)를 사용하여 대상 Bruteforce
- `kiterunner brute https://example.com -w 경로/대상/워드리스트.txt` : 사용자 지정 워드리스트를 사용하여 대상 Bruteforce
- `kiterunner brute https://example.com -w 경로/대상/dirsearch.txt -D -e json,txt` : dirsearch 형식의 워드리스트와 확장자 치환 기능을 사용하여 Bruteforce
- `kiterunner brute https://example.com -w 경로/대상/워드리스트.txt -e aspx,ashx -o json` : 특정 파일 확장자를 추가하여 Bruteforce하고 결과를 JSON 형식으로 출력
- `kiterunner brute 경로/대상/targets.txt -w 경로/대상/워드리스트.txt -x 5 -j 100` : 파일에 정의된 대상 목록을 사용자 지정 동시성 설정으로 Bruteforce
- `kiterunner brute https://example.com -w 경로/대상/워드리스트.txt --ignore-length 100-105` : 특정 응답 콘텐츠 길이를 무시하면서 Bruteforce
- `kiterunner brute https://example.com -w 경로/대상/워드리스트.txt -H "Authorization: Bearer token"` : 사용자 지정 HTTP 헤더를 사용하여 Bruteforce
- `kiterunner brute 경로/대상/targets.txt -w 경로/대상/워드리스트.txt --fail-status-codes 400,401,404` : 특정 실패 상태 코드 응답을 제외하면서 대상 목록 Bruteforce

## Links
- More information: https://github.com/assetnote/kiterunner#usage
