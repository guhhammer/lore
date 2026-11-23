# mongodump

MongoDB 인스턴스의 내용을 내보내는 유틸리티.

## Usage
- `mongodump` : 모든 데이터베이스의 덤프 생성 (파일은 "dump"라는 폴더에 저장됨)
- `mongodump -o 경로/대상/폴더` : 덤프의 출력 위치 지정
- `mongodump -d 데이터베이스_이름` : 특정 데이터베이스의 덤프 생성
- `mongodump -c 컬렉션_이름 -d 데이터베이스_이름` : 특정 데이터베이스 내의 특정 컬렉션 덤프 생성
- `mongodump -h 호스트 --port 포트` : 주어진 포트에서 실행 중인 특정 호스트에 연결하여 덤프 생성
- `mongodump -u 사용자_명 데이터베이스 -p` : 특정 사용자 명으로 데이터베이스의 덤프 생성; 사용자에게 비밀번호 입력이 요청됨
- `mongodump --uri 연결_문자열` : 특정 인스턴스에서 덤프 생성; 호스트, 사용자, 비밀번호 및 데이터베이스가 연결 문자열에 정의됨

## Links
- More information: https://www.mongodb.com/docs/database-tools/mongodump/
