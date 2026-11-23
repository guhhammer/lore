# pg_resetwal

PostgreSQL 데이터베이스 클러스터의 write-ahead log 및 제어 정보를 초기화.

## Usage
- `pg_resetwal -D 경로/대상/데이터` : 지정한 데이터 디렉터리의 WAL 및 제어 정보 초기화
- `pg_resetwal -D 경로/대상/데이터 -n` : dry run 수행
- `pg_resetwal -D 경로/대상/데이터 -f` : WAL 및 제어 정보 강제 초기화
- `pg_resetwal -?` : 도움말 표시
- `pg_resetwal -V` : 버전 표시

## Links
- More information: https://www.postgresql.org/docs/current/app-pgresetwal.html
