# kubectl get

Kubernetes 객체 또는 리소스 가져오기.

## Usage
- `kubectl get ns` : 현재 클러스터의 모든 네임스페이스 조회
- `kubectl get no -n 네임스페이스` : 특정 네임스페이스의 노드 조회
- `kubectl get po -n 네임스페이스` : 특정 네임스페이스의 Pod 조회
- `kubectl get deploy -n 네임스페이스` : 특정 네임스페이스의 Deployment 조회
- `kubectl get svc -n 네임스페이스` : 특정 네임스페이스의 Service 조회
- `kubectl get persistentvolumeclaims|secret|...` : 기타 Kubernetes 리소스 조회
- `kubectl get all -A` : 모든 네임스페이스의 주요 리소스 조회
- `kubectl get -f 경로/대상/manifest.yaml` : YAML 매니페스트 파일에 정의된 Kubernetes 오브젝트 조회

## Links
- More information: https://kubernetes.io/docs/reference/kubectl/generated/kubectl_get/
