# pg_upgrade

PostgreSQL 데이터베이스 클러스터를 새로운 메이저 버전으로 업그레이드.

## Usage
- `pg_upgrade -c -b 경로/대상/오래된_bin -B 경로/대상/새로운_bin -d 경로/대상/오래된_데이터 -D 경로/대상/새로운_데이터` : 업그레이드 전에 클러스터 검사
- `pg_upgrade -b 경로/대상/오래된_bin -B 경로/대상/새로운_bin -d 경로/대상/오래된_데이터 -D 경로/대상/새로운_데이터` : 실제 업그레이드 수행
- `pg_upgrade -j 여러_jobs -b 경로/대상/오래된_bin -B 경로/대상/새로운_bin -d 경로/대상/오래된_데이터 -D 경로/대상/새로운_데이터` : 업그레이드 중 여러 병렬 작업 사용
- `pg_upgrade -p port -P port -b 경로/대상/오래된_bin -B 경로/대상/새로운_bin -d 경로/대상/오래된_데이터 -D 경로/대상/새로운_데이터` : 기존 및 새로운 PostgreSQL 포트 지정
- `pg_upgrade -k -b 경로/대상/오래된_bin -B 경로/대상/새로운_bin -d 경로/대상/오래된_데이터 -D 경로/대상/새로운_데이터` : 새로운 클러스터로 파일을 복사하는 대신 하드 링크 사용
- `pg_upgrade -?` : 도움말 표시

## Links
- More information: https://www.postgresql.org/docs/current/pgupgrade.html
