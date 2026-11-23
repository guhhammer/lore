# stern

Kubernetes의 여러 팟 및 컨테이너 로그를 동시에 확인.

## Usage
- `stern .` : 현재 네임스페이스 내의 모든 팟 로그 확인
- `stern . --container-state running|waiting|terminated` : 특정 상태의 모든 팟 로그 확인
- `stern 팟_쿼리` : 주어진 정규 표현식과 일치하는 모든 팟 로그 확인
- `stern 팟_쿼리 -A` : 모든 네임스페이스에서 일치하는 팟 로그 확인
- `stern 팟_쿼리 -s 15m` : 15분 전부터 일치하는 팟 로그 확인
- `stern 팟_쿼리 -l release=canary` : 특정 레이블이 있는 일치하는 팟 로그 확인

## Links
- More information: https://github.com/stern/stern#usage
