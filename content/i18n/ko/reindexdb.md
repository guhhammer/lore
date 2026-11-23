# reindexdb

PostgreSQL 데이터베이스의 인덱스를 다시 생성.

## Usage
- `reindexdb 데이터베이스_이름` : 지정한 데이터베이스의 인덱스 다시 생성
- `reindexdb 데이터베이스_이름 -h 호스트명 -p 포트 -U 호스트명` : 연결 옵션을 지정하여 데이터베이스의 인덱스 다시 생성
- `reindexdb -a` : 모든 데이터베이스의 인덱스 다시 생성
- `reindexdb 데이터베이스_이름 -t 테이블_이름` : 데이터베이스의 지정한 테이블 인덱스 다시 생성
- `reindexdb 데이터베이스_이름 -i 인덱스_이름` : 데이터베이스의 지정한 인덱스 다시 생성
- `reindexdb 데이터베이스_이름 -S 스키마_이름` : 데이터베이스의 지정한 스키마 인덱스 다시 생성
- `reindexdb 데이터베이스_이름 -v` : 자세한 출력을 표시하며 인덱스 다시 생성
- `reindexdb 데이터베이스_이름 -j 작업_개수` : 여러 병렬 작업을 사용하여 데이터베이스의 인덱스 다시 생성

## Links
- More information: https://www.postgresql.org/docs/current/app-reindexdb.html
