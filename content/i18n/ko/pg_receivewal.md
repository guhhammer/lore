# pg_receivewal

PostgreSQL 클러스터의 write-ahead log을 스트리밍함.

## Usage
- `pg_receivewal -D 디렉토리` : WAL을 로컬 디렉토리로 스트리밍 (최소 필수 옵션)
- `pg_receivewal -v -h 호스트 -p 포트 -U 사용자명 -D 디렉토리` : 호스트, 포트, 사용자명을 지정하고 자세한 출력과 함께 WAL 스트리밍
- `pg_receivewal -S slot_name --create-slot -h 호스트 -p 포트 -U 사용자명 -D 디렉토리` : 복제 슬롯(replication slot) 사용 (필요 시 생성)
- `pg_receivewal -E lsn -D 디렉토리 -h 호스트 -p 포트 -U 사용자명` : 지정한 WAL 위치 (LSN)까지 스트리밍
- `pg_receivewal -n -D 디렉토리 -h 호스트 -p 포트 -U 사용자명` : 오류 발생 시 재시도(루프) 비활성화
- `pg_receivewal --synchronous -D 디렉토리 -h 호스트 -p 포트 -U 사용자명` : 동기식 flush 수행 (WAL 데이터를 즉시 디스크에 기록)
- `pg_receivewal -Z level|method -D 디렉토리 -h 호스트 -p 포트 -U 사용자명` : WAL 출력을 압축하여 저장 (`gzip`, 압출 레벨 0-9)
- `pg_receivewal -s 초 -D 디렉토리 -h 호스트 -p 포트 -U 사용자명` : 상태 보고 간격 설정

## Links
- More information: https://www.postgresql.org/docs/current/app-pgreceivewal.html
