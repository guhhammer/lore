# pg_checksums

PostgreSQL 데이터베이스 클러스터의 데이터 체크섬을 활성화, 비활성화하거나 검사.

## Usage
- `pg_checksums -D 경로/대상/데이터` : 데이터베이스 클러스터의 데이터 체크섬 검사
- `pg_checksums -e -D 경로/대상/데이터` : 데이터베이스 클러스터의 데이터 체크섬 활성화
- `pg_checksums -d -D 경로/대상/데이터` : 데이터베이스 클러스터의 데이터 체크섬 비활성화
- `pg_checksums -v -D 경로/대상/데이터` : 자세한 출력과 함께 데이터 체크섬 검사
- `pg_checksums -P -D 경로/대상/데이터` : 진행 상황을 표시하며 데이터 체크섬 검사
- `pg_checksums -?` : 도움말 표시

## Links
- More information: https://www.postgresql.org/docs/current/app-pgchecksums.html
