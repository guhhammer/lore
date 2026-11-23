# kubectl explain

Kubernetes API 리소스의 문서, 사용 가능한 필드 및 설명 표시.

## Usage
- `kubectl explain pods|nodes|deployments|...` : 특정 리소스 문서 보기
- `kubectl explain pod.spec.containers` : 객체의 하위 리소스 또는 특정 필드의 문서 표시
- `kubectl explain ingress.v1.networking.k8s.io` : 특정 버전의 리소스 문서 표시
- `kubectl explain po --recursive` : 리소스의 모든 필드를 재귀적으로 표시
- `kubectl explain --help` : 도움말 표시

## Links
- More information: https://kubernetes.io/docs/reference/kubectl/generated/kubectl_explain/
