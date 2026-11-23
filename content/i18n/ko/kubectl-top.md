# kubectl top

Kubernetes 노드 또는 Pod의 리소스 사용량 조회.

## Usage
- `kubectl top no` : 모든 노드의 리소스 사용량 조회
- `kubectl top no 노드_이름` : 특정 노드의 리소스 사용량 조회
- `kubectl top po` : 모든 Pod의 리소스 사용량 조회
- `kubectl top po pod_이름` : 특정 Pod의 리소스 사용량 조회
- `kubectl top po -n 네임스페이스_이름` : 특정 네임스페이스의 모든 Pod 리소스 사용량 조회
- `kubectl top po --containers` : pod 내 모든 컨테이너의 리소스 사용량 조회
- `kubectl top po -l 키=값` : 특정 라벨을 가진 Pod들의 리소스 사용량 조회

## Links
- More information: https://kubernetes.io/docs/reference/kubectl/generated/kubectl_top/
