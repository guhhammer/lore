# doctl databases pool

데이터베이스 클러스터의 연결 풀을 관리.

## Usage
- `doctl d p 명령어 -t 액세스_토큰` : 액세스 토큰을 사용하여 `doctl databases pool` 명령을 실행
- `doctl d p g 데이터베이스_아이디 풀_이름` : 데이터베이스 연결 풀에 대한 정보 검색
- `doctl d p ls 데이터베이스_아이디` : 데이터베이스 클러스터에 대한 연결 풀을 나열
- `doctl d p c 데이터베이스_아이디 풀_이름 --db 새로운_풀_이름 --size 풀_크기` : 데이터베이스에 대한 연결 풀을 생성
- `doctl d p rm 데이터베이스_아이디 풀_이름` : 데이터베이스에 대한 연결 풀을 삭제

## Links
- More information: https://docs.digitalocean.com/reference/doctl/reference/databases/pool/
