# doctl kubernetes cluster

Kubernetes 클러스터를 관리하고 클러스터와 관련된 구성 옵션을 봄.

## Usage
- `doctl k c c --count 3 --region nyc1 --size s-1vcpu-2gb --version latest 클러스터_이름` : kubernetes 클러스터 생성
- `doctl k c ls` : 모든 Kubernetes 클러스터 나열
- `doctl k c cfg s 클러스터_이름` : kubeconfig를 가져와 저장
- `doctl k c gu 클러스터_이름` : 사용 가능한 업그레이드 확인
- `doctl k c upgrade 클러스터_이름` : 클러스터를 새로운 Kubernetes 버전으로 업그레이드
- `doctl k c d 클러스터_이름` : 클러스터 삭제

## Links
- More information: https://docs.digitalocean.com/reference/doctl/reference/kubernetes/cluster/
