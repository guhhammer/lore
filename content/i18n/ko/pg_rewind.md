# pg_rewind

PostgreSQL 데이터 디렉토리를 동일한 기반에서 분기된 다른 데이터 디렉토리와 동기화.

## Usage
- `pg_rewind -D 경로/대상/대상_데이터 --source-pgdata 경로/대상/원본_데이터` : 대상 데이터 디렉토리를 원본 데이터 디렉토리와 동기화
- `pg_rewind -D 경로/대상/대상_데이터 --source-server connstr` : 연결 문자열을 사용하여 원본 서버와 대상 데이터 디렉터리를 동기화
- `pg_rewind -D 경로/대상/대상_데이터 --source-pgdata 경로/대상/원본_데이터 -n` : dry run 실행
- `pg_rewind -D 경로/대상/대상_데이터 --source-pgdata 경로/대상/원본_데이터 -P` : 진행 상황을 표시하며 동기화
- `pg_rewind -?` : 도움말 표시

## Links
- More information: https://www.postgresql.org/docs/current/app-pgrewind.html
