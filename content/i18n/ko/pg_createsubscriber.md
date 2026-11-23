# pg_createsubscriber

물리적 복제본을 새로운 논리적 복제본으로 변환.

## Usage
- `pg_createsubscriber -d 데이터베이스_이름 -D 경로/대상/데이터 -P connstr` : 특정 데이터베이스의 물리적 복제본을 논리적 복제본으로 변환
- `pg_createsubscriber -n -d 데이터베이스_이름 -D 경로/대상/데이터 -P connstr` : dry run 수행 (대상 디렉터리를 변경하지 않음)
- `pg_createsubscriber -T -d 데이터베이스_이름 -D 경로/대상/데이터 -P connstr` : 구독에 Two-phase commit 활성화
- `pg_createsubscriber -v -d 데이터베이스_이름 -D 경로/대상/데이터 -P connstr` : 자세한 출력과 함께 변환
- `pg_createsubscriber -?` : 도움말 표시

## Links
- More information: https://www.postgresql.org/docs/current/app-pgcreatesubscriber.html
