# pg_basebackup

실행 중인 PostgreSQL 클러스터의 베이스 백업을 생성. 전체 또는 증분 백업, 특정 시점 복구, 복제 스탠바이 서버 구축에 사용됨.

## Usage
- `pg_basebackup -h 호스트 -D 경로/대상/백업_디렉토리` : 원격 PostgreSQL 서버의 베이스 백업 생성
- `pg_basebackup -h 호스트 -D 경로/대상/백업_디렉토리 -P` : 진행 상황을 표시하며 백업 생성
- `pg_basebackup -D 경로/대상/백업_디렉토리 -F t -z` : tar 형식의 (`gzip`) 압축 파일 생성
- `pg_basebackup -D 경로/대상/백업_디렉토리 -i 경로/대상/오래된_manifest` : 이전 manifest 파일을 사용하여 증분 백업 생성
- `pg_basebackup -D 경로/대상/백업_디렉토리 -R` : 스탠바이 서버 구성을 위한 복구 설정 파일 생성
- `pg_basebackup -D 경로/대상/백업_디렉토리 -T 경로/대상/오래된_tablespace=경로/대상/새로운_tablespace` : 백업 중 tablespace 위치 변경
- `pg_basebackup -D 경로/대상/백업_디렉토리 -r 100M` : 서버 부하를 줄이기 위해 전송 속도 제한
- `pg_basebackup -D 경로/대상/백업_디렉토리 -X stream` : 백업과 동시에 WAL 로그를 스트리밍

## Links
- More information: https://www.postgresql.org/docs/current/app-pgbasebackup.html
