# doctl databases user

데이터베이스 사용자에 대한 세부 정보를 보고 생성.

## Usage
- `doctl d u 명령어 -t access_token` : 액세스 토큰을 사용하여 `doctl databases user` 명령을 실행
- `doctl d u g 데이터베이스_아이디 사용자_이름` : 데이터베이스 사용자에 대한 세부 정보를 검색
- `doctl d u ls 데이터베이스_아이디` : 특정 데이터베이스에 대한 데이터베이스 사용자 목록을 검색
- `doctl d u rs 데이터베이스_아이디 사용자_이름` : 특정 사용자의 인증 비밀번호를 재설정
- `doctl d u rs 데이터베이스_아이디 사용자_이름 caching_sha2_password|mysql_native_password` : 특정 사용자에 대한 MySQL 인증 플러그인 재설정
- `doctl d u c 데이터베이스_아이디 사용자_이름` : 주어진 사용자 이름으로 주어진 데이터베이스에 사용자를 생성
- `doctl d u rm 데이터베이스_아이디 사용자_이름` : 주어진 사용자 이름을 가진 주어진 데이터베이스에서 사용자를 삭제

## Links
- More information: https://docs.digitalocean.com/reference/doctl/reference/databases/user/
