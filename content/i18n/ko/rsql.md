# rsql

터미널에서 다양한 데이터베이스 및 기타 데이터 소스에 연결하는 SQL 클라이언트.

## Usage
- `rsql` : 대화형 모드로 실행
- `rsql --url "postgresql://user:pass@localhost/mydb"` : 데이터베이스에 연결 (예: PostgreSQL)
- `rsql --url "postgresql://user:pass@localhost/db?sslmode=require"` : SSL을 사용하여 PostgreSQL 데이터베이스에 연결
- `rsql --url "mysql://user:pass@localhost/db?charset=utf8mb4"` : 지정한 문자 집합으로 MySQL 데이터베이스에 연결
- `rsql --url "sqlite://database.db" -- "SELECT * FROM users LIMIT 10"` : 쿼리를 실행하고 종료
- `rsql --url "sqlite://db.sqlite" --format json` : 기본 출력 형식 설정
- `rsql --url "delimited://data.txt?separator=|&has_header=true"` : 파일 데이터 소스에 연결하고 사용자 지정 구분자 사용

## Links
- More information: https://github.com/theseus-rs/rsql
