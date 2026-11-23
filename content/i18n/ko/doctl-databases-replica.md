# doctl databases replica

데이터베이스 클러스터와 연결된 읽기 전용 복제본을 관리.

## Usage
- `doctl d r 명령어 -t 액세스_토큰` : 액세스 토큰을 사용하여 `doctl databases replica` 명령을 실행
- `doctl d r g 데이터베이스_아이디 복제본_이름` : 읽기 전용 데이터베이스 복제본에 대한 정보를 검색
- `doctl d r ls 데이터베이스_아이디` : 읽기 전용 데이터베이스 복제본 목록 검색
- `doctl d r c 데이터베이스_아이디 복제본_이름` : 읽기 전용 데이터베이스 복제본 생성
- `doctl d r rm 데이터베이스_아이디 복제본_이름` : 읽기 전용 데이터베이스 복제본 삭제

## Links
- More information: https://docs.digitalocean.com/reference/doctl/reference/databases/replica/
