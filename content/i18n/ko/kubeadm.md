# kubeadm

Kubernetes 클러스터를 생성하고 관리하기 위한 명령줄 인터페이스.

## Usage
- `kubeadm init` : kubernetes 마스터 노드 생성
- `kubeadm join --token 토큰` : kubernetes 워커 노드를 부트스트랩하고 클러스터에 가입
- `kubeadm token create --ttl 12h0m0s` : TTL이 12시간인 새로운 부트스트랩 토큰 생성
- `kubeadm upgrade plan` : kubernetes 클러스터가 업그레이드 가능한지와 사용 가능한 버전 확인
- `kubeadm upgrade apply 버전` : 지정된 버전으로 Kubernetes 클러스터 업그레이드
- `kubeadm config view` : 클러스터의 구성이 포함된 kubeadm ConfigMap 보기
- `kubeadm reset` : 'kubeadm init' 또는 'kubeadm join'으로 호스트에 적용된 변경사항 되돌리기

## Links
- More information: https://kubernetes.io/docs/reference/setup-tools/kubeadm/
