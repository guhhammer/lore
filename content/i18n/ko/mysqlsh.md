# mysqlsh

SQL, JavaScript, Python을 지원하는 MySQL의 고급 명령줄 클라이언트. InnoDB 클러스터 및 문서 저장소 컬렉션 관리를 위한 기능 제공.

## Usage
- `mysqlsh` : 대화형 모드로 MySQL Shell 시작
- `mysqlsh --user 사용자_명 --host 호스트_명 --port 포트` : mySQL 서버에 연결
- `mysqlsh --user 사용자_명 --execute 'sql_문'` : 서버에서 SQL 문을 실행하고 종료
- `mysqlsh --js` : javaScript 모드로 MySQL Shell 시작
- `mysqlsh --py` : python 모드로 MySQL Shell 시작
- `mysqlsh --import 경로/대상/파일.json --schema 스키마_명 --collection 컬렉션_명` : JSON 문서를 MySQL 컬렉션에 가져오기
- `mysqlsh --verbose` : 상세 출력 활성화

## Links
- More information: https://manned.org/mysqlsh
