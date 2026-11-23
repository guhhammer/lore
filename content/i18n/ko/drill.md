# drill

다양한 DNS 쿼리를 수행.

## Usage
- `drill example.com` : 호스트 이름 (A 레코드)과 연결된 IP를 조회
- `drill mx example.com` : 특정 도메인 이름 (MX 레코드)과 연결된 메일 서버를 조회
- `drill any example.com` : 특정 도메인 이름에 대한 모든 유형의 레코드를 가져옴
- `drill example.com @8.8.8.8` : 쿼리할 대체 DNS 서버를 지정
- `drill -x 8.8.8.8` : IP 주소 (PTR 레코드)에 대해 역방향 DNS 조회를 수행
- `drill -TD example.com` : 루트 서버부터 도메인 이름까지 DNSSEC 추적을 수행
- `drill -s dnskey example.com` : 도메인 이름에 대한 DNSKEY 레코드 표시

## Links
- More information: https://manned.org/drill
