# doctl databases db

데이터베이스 클러스터에서 제공하는 데이터베이스를 관리.

## Usage
- `doctl d db 명령어 -t 액세스_토큰` : 액세스 토큰을 사용하여 `doctl databases db` 명령을 실행
- `doctl d db g 데이터베이스_아이디 데이터베이스_이름` : 특정 데이터베이스 클러스터에서 호스팅되는 특정 데이터베이스의 이름을 검색
- `doctl d db ls 데이터베이스_아이디` : 특정 데이터베이스 클러스터 내에서 호스팅되는 기존 데이터베이스를 나열
- `doctl d db c 데이터베이스_아이디 데이터베이스_이름` : 주어진 데이터베이스 클러스터에 주어진 이름을 가진 데이터베이스를 생성
- `doctl d db rm 데이터베이스_아이디 데이터베이스_이름` : 주어진 데이터베이스 클러스터에 주어진 이름을 가진 데이터베이스를 삭제

## Links
- More information: https://docs.digitalocean.com/reference/doctl/reference/databases/db/
