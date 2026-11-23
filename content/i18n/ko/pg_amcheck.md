# pg_amcheck

하나 이상의 PostgreSQL 데이터베이스에서 손상 여부를 검사.

## Usage
- `pg_amcheck 데이터베이스_이름` : 특정 데이터베이스 검사
- `pg_amcheck -a` : 모든 데이터베이스 검사
- `pg_amcheck -d 패턴` : 지정한 패턴과 일치하는 데이터베이스 검사
- `pg_amcheck -t 패턴 dbname` : 데이터베이스 내 특정 테이블 검사
- `pg_amcheck -s 패턴 dbname` : 데이터베이스 내 특정 스키마 검사
- `pg_amcheck -?` : 도움말 표시

## Links
- More information: https://www.postgresql.org/docs/current/app-pgamcheck.html
