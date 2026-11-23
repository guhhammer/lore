# knotc

Knot DNS 서버 제어.

## Usage
- `knotc zone-begin zone` : zone 편집 시작
- `knotc zone-set zone 서브도메인 3600 A ip_주소` : TTL이 3600인 A 레코드 설정
- `knotc zone-commit zone` : zone 편집 완료
- `knotc zone-read zone` : 현재 zone 데이터 얻기
- `knotc conf-read server` : 현재 서버 구성 가져오기

## Links
- More information: https://www.knot-dns.cz/docs/latest/html/man_knotc.html
