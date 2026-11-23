# pg_archivecleanup

PostgreSQL의 오래된 WAL 아카이브 파일을 정리.

## Usage
- `pg_archivecleanup 경로/대상/아카이브 경로/대상/wal_파일` : 지정한 WAL 파일까지 아카이브 디렉터리 정리
- `pg_archivecleanup -n 경로/대상/아카이브 경로/대상/wal_파일` : dry run 수행 (실제 삭제하지 않고 삭제 대상 파일만 표시)
- `pg_archivecleanup -x 확장자 경로/대상/아카이브 경로/대상/wal_파일` : 삭제 여부를 결정하기 전에 파일 확장자 제거
- `pg_archivecleanup -b 경로/대상/아카이브 경로/대상/wal_파일` : 백업 히스토리 파일도 함께 삭제
- `pg_archivecleanup -d 경로/대상/아카이브 경로/대상/wal_파일` : 디버그 로그 출력 활성화

## Links
- More information: https://www.postgresql.org/docs/current/pgarchivecleanup.html
