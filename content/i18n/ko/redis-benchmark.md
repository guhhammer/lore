# redis-benchmark

Redis 서버의 성능을 테스트.

## Usage
- `redis-benchmark` : 전체 벤치마크 실행
- `redis-benchmark -h 호스트 -p 포트 -a 비밀번호` : 특정 Redis 서버에서 벤치마크 실행
- `redis-benchmark -h 호스트 -p 포트 -t set,lpush -n 100000` : 기본 100000 요청으로 테스트의 일부 실행
- `redis-benchmark -n 100000 script load "redis.call('set', 'foo', 'bar')"` : 특정 스크립트로 실행
- `redis-benchmark -t set -r 100000` : 100000개의 random 키를 사용하여 벤치마크 실행
- `redis-benchmark -n 1000000 -t set,get -P 16` : 16개의 명령으로 Pipelining하여 벤치마크 실행
- `redis-benchmark -q` : quietly 실행하여 초당 쿼리 결과만 표시
