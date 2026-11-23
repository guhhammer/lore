# pg_recvlogical

PostgreSQL 논리 디코딩 스트림을 제어.

## Usage
- `pg_recvlogical -d 데이터베이스_이름 -S 슬롯_이름 --create-slot` : 새로운 논리 복제 슬롯 생성
- `pg_recvlogical -d 데이터베이스_이름 -S 슬롯_이름 --start -f 파일이름` : 논리 복제 슬롯의 변경 사항을 파일로 스트리밍
- `pg_recvlogical -d 데이터베이스_이름 -S 슬롯_이름 --drop-slot` : 논리 복제 슬롯 삭제
- `pg_recvlogical -d 데이터베이스_이름 -S 슬롯_이름 --create-slot -t` : two-phase commit이 활성화된 논리 복제 슬롯 생성
- `pg_recvlogical -?` : 도움말 표시

## Links
- More information: https://www.postgresql.org/docs/current/app-pgrecvlogical.html
