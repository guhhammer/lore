# kubectl delete

Kubernetes 리소스 삭제.

## Usage
- `kubectl delete po 포드_이름` : 특정 포드 삭제
- `kubectl delete deploy 배포_이름` : 특정 배포 삭제
- `kubectl delete no 노드_이름` : 특정 노드 삭제
- `kubectl delete po --all -n 네임스페이스` : 지정된 네임스페이스의 모든 포드 삭제
- `kubectl delete deploy,svc --all -n 네임스페이스` : 지정된 네임스페이스의 모든 배포 및 서비스 삭제
- `kubectl delete no --all` : 모든 노드 삭제
- `kubectl delete -f 경로/대상/매니페스트.yaml` : YAML 매니페스트에 정의된 리소스 삭제

## Links
- More information: https://kubernetes.io/docs/reference/kubectl/generated/kubectl_delete/
