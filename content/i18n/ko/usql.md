# usql

SQL 데이터베이스를 위한 범용 CLI 인터페이스.

## Usage
- `usql sqlserver|mysql|postgres|sqlite3|...://사용자_명:비밀번호@호스트:포트/데이터베이스_명` : 특정 데이터베이스에 연결
- `usql -f 경로/대상/query.sql` : 파일에서 명령 실행
- `usql -c "sql_명령"` : 특정 SQL 명령 실행
- `명령` : [대화형] `usql` 프롬프트에서 SQL 명령 실행
- `\d` : [대화형] 데이터베이스 스키마 표시
- `\g 경로/대상/결과_파일` : [대화형] 특정 파일로 쿼리 결과 내보내기
- `\copy 경로/대상/데이터.csv 테이블_명` : [대화형] CSV 파일에서 특정 테이블로 데이터 가져오기

## Links
- More information: https://github.com/xo/usql#using
