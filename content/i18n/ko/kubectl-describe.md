# kubectl describe

Kubernetes 객체 및 리소스의 세부 정보 표시.

## Usage
- `kubectl describe po -n 네임스페이스` : namespace의 포드 세부 정보 표시
- `kubectl describe no -n 네임스페이스` : namespace의 노드 세부 정보 표시
- `kubectl describe po 포드_이름 -n 네임스페이스` : namespace의 특정 포드 세부 정보 표시
- `kubectl describe no 노드_이름 -n 네임스페이스` : namespace의 특정 노드 세부 정보 표시
- `kubectl describe -f 경로/대상/manifest.yaml` : YAML 매니페스트 file에 정의된 Kubernetes 객체의 세부 정보 표시

## Links
- More information: https://kubernetes.io/docs/reference/kubectl/generated/kubectl_describe/
