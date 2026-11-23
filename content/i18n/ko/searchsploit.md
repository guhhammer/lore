# searchsploit

Exploit Database에서 익스플로잇, 쉘코드 및/또는 논문을 검색. 알려진 버전 번호가 검색어로 사용되면, 해당 버전에 대한 익스플로잇뿐만 아니라 지정된 버전 범위를 포함하는 다른 익스플로잇도 표시됩니다.

## Usage
- `searchsploit 검색어` : 익스플로잇, 쉘코드 또는 논문 검색
- `searchsploit sudo 1.8.27` : 특정 버전을 검색, 예: sudo 버전 1.8.27
- `searchsploit -w 검색어` : 발견된 리소스의 exploit-db 링크 표시
- `searchsploit -m 익스플로잇_번호` : 리소스를 현재 디렉토리로 복사 (mirror) (익스플로잇 번호 필요)
- `searchsploit -x 익스플로잇_번호` : `$PAGER` 환경 변수에 정의된 페이지 뷰어를 사용하여 리소스를 examine
- `searchsploit -u` : 로컬 Exploit Database u업데이트
- `searchsploit --cve 2021-44228` : common vulnerabilities and exposures (CVE) 값 검색
- `searchsploit --nmap 경로/대상/nmap-output.xml` : `nmap`의 XML 출력에서 서비스 버전 (`nmap -sV -oX nmap-output.xml`)으로 알려진 익스플로잇 확인

## Links
- More information: https://www.exploit-db.com/searchsploit
