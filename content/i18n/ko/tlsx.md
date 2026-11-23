# tlsx

TLS 기반 데이터를 수집하고 분석하는 빠르고 설정 가능한 TLS 정보 수집 도구.

## Usage
- `tlsx -u 호스트1,호스트2,... -tv -cipher` : 하나 이상의 호스트에 대해 TLS 버전과 암호화 스위트 표시
- `tlsx -u 호스트 -ve -ce` : 호스트가 지원하는 TLS 버전과 암호화 스위트 열거 및 표시
- `tlsx -l 경로/대상/호스트.txt -ex -ss -mm -re -un` : 파일의 호스트 목록을 스캔하고 만료됨/자체 서명/불일치/폐기됨/신뢰되지 않음 인증서를 가진 호스트 표시
- `tlsx -l 경로/대상/호스트.txt -c 300 -timeout 5 -retry 3 -delay 200ms -wc` : 와일드카드 SSL 인증서를 검사할 때, 호스트별 동시성, 타이아웃, 재시도 횟수 및 지연 시간 조정
- `tlsx -u 호스트 -dns` : SSL 인증서 응답에서 공유한 호스트 이름 표시
- `tlsx -u 호스트 -san -j -o 경로/대상/파일.json` : 호스트의 TLS 인증서에서 SAN(Subject Alternative Names)을 표시하고, JSON 형식으로 파일에 저장
- `tlsx -hc` : `tlsx` 자체 상태 점검 수행

## Links
- More information: https://github.com/projectdiscovery/tlsx#usage
