# redis-cli

Redis 서버에 연결합니다.

## Usage
- `redis-cli` : 로컬 서버에 연결
- `redis-cli -h 호스트` : 기본 포트(6379)로 원격 서버에 연결
- `redis-cli -h 호스트 -p 포트` : 포트 번호를 지정하여 원격 서버에 연결
- `redis-cli -u uri` : URI를 지정하여 원격 서버에 연결
- `redis-cli -a 비밀번호` : 비밀번호 지정
- `redis-cli redis_명령` : redis 명령 실행
- `redis-cli -c` : 로컬 클러스터에 연결

## Links
- More information: https://redis.io/docs/latest/develop/
