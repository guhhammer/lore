# dropdb

PostgreSQL 데이터베이스를 삭제하는 도구. SQL 명령어 `DROP DATABASE`를 감싼 간단한 래퍼.

## Usage
- `dropdb 데이터베이스_이름` : 데이터베이스 삭제
- `dropdb -i 데이터베이스_이름` : 삭제 전 확인 프롬프트 표시
- `dropdb -U 사용자명 데이터베이스_이름` : 특정 사용자로 접속하여 데이터베이스 삭제
- `dropdb -W 데이터베이스_이름` : 데이터베이스 접속 전 비밀번호 입력을 강제
- `dropdb -w 데이터베이스_이름` : 데이터베이스 접속 전 비밀번호 입력 프롬프트 비활성화
- `dropdb -h 호스트 데이터베이스_이름` : 서버 호스트 지정
- `dropdb -p 포트 데이터베이스_이름` : 서버 포트 지정
- `dropdb -f 데이터베이스_이름` : 기존 연결을 종료한 후 데이터베이스 삭제 시도

## Links
- More information: https://www.postgresql.org/docs/current/app-dropdb.html
