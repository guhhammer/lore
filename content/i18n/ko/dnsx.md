# dnsx

여러 DNS 쿼리를 실행하기 위한 빠르고 목적이 다양한 DNS 도구 키트. 참고: 어떤 경우에는 `dnsx`에 대한 입력이 `stdin` (파이프 `|`)를 통해 전달되어야 함.

## Usage
- `echo example.com | dnsx -a -re` : (하위)도메인의 A 레코드를 쿼리하고 수신된 응답([re]sponse)을 표시
- `dnsx <<<example.com -recon -re` : 모든 DNS 레코드(A, AAAA, CNAME, NS, TXT, SRV, PTR, MX, SOA, AXFR, CAA)를 쿼리
- `echo example.com | dnsx -re -a|aaaa|cname|ns|txt|srv|ptr|mx|soa|any|axfr|caa` : 특정 유형의 DNS 레코드를 쿼리
- `echo example.com | dnsx -ro` : 응답(response)만 (only) 출력 (쿼리된 도메인이나 하위 도메인은 표시하지 않음)
- `echo example.com | dnsx -debug|raw -r 1.1.1.1,8.8.8.8,... -retry number` : 쿼리의 원시 응답을 표시하고, 실패에 대한 시도를 라도 재시도할 resolvers를 지정
- `dnsx -d FUZZ.example.com -w 경로/대상/단어목록.txt -re` : 자리 표시자를 사용한 무차별 대입 DNS 레코드
- `dnsx -d 경로/대상/도메인.txt -w 경로/대상/단어목록.txt -re -o 경로/대상/출력.txt -nc` : DNS 도메인(domains) 및 단어 목록의 무차별 대입 DNS 레코드를 색상 코드가 없는(no color) 파일에 출력(output) 결과를 추가
- `subfinder -silent -d example.com | dnsx -cname -re -rl 숫자` : 초당 DNS 쿼리 속도를 제한하여(rate limiting) 지정된 하위 도메인 목록에 대한 `CNAME` 레코드를 추출

## Links
- More information: https://docs.projectdiscovery.io/opensource/dnsx/usage
