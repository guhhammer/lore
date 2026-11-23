# cli53

Amazon Route 53용 명령줄 도구.

## Usage
- `cli53 list` : 도메인 나열
- `cli53 create 도메인주소.com --comment "코멘트"` : 도메인 생성
- `cli53 export 도메인주소.com` : 바인드 존 파일을 `stdout`으로 내보내기
- `cli53 rc 도메인주소.com 'www 300 CNAME lb'` : 동일한 영역의 상대 레코드를 가리키는 `www` 하위 도메인을 생성
- `cli53 rc 도메인주소.com 'www 300 CNAME lb.externalhost.com.'` : 외부 주소를 가리키는 `www` 하위 도메인을 생성 (점으로 끝나야 함)
- `cli53 rc 도메인주소.com 'www 300 A 150.130.110.1'` : IP 주소를 가리키는 `www` 하위 도메인을 생성
- `cli53 rc --replace 'www 300 A 150.130.110.2'` : 다른 IP를 가리키는 `www` 하위 도메인을 교체
- `cli53 rd 도메인주소.com www A` : A 레코드 삭제

## Links
- More information: https://github.com/barnybug/cli53
