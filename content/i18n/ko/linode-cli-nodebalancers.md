# linode-cli nodebalancers

Linode NodeBalancer 관리.

## Usage
- `linode-cli nodebalancers list` : 모든 NodeBalancer 나열
- `linode-cli nodebalancers create --region 지역` : 새 NodeBalancer 생성
- `linode-cli nodebalancers view nodebalancer_id` : 특정 NodeBalancer의 세부 정보 보기
- `linode-cli nodebalancers update nodebalancer_id --label 새_라벨` : 기존 NodeBalancer 업데이트
- `linode-cli nodebalancers delete nodebalancer_id` : nodeBalancer 삭제
- `linode-cli nodebalancers configs list nodebalancer_id` : nodeBalancer의 구성 목록 나열
- `linode-cli nodebalancers configs create nodebalancer_id --port 포트 --protocol 프로토콜` : nodeBalancer에 새 구성 추가
