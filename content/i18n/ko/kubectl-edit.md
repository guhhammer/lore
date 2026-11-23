# kubectl edit

Kubernetes 리소스 편집.

## Usage
- `kubectl edit po/pod_이름` : 기본 네임스페이스의 Pod 편집
- `kubectl edit deploy/deployment_이름` : 기본 네임스페이스의 Deployment 편집
- `kubectl edit svc/service_이름` : 기본 네임스페이스의 Service 편집
- `kubectl edit 리소스 -n 네임스페이스` : 특정 네임스페이스의 해당 리소스 전체 편집
- `KUBE_EDITOR=nano kubectl edit 리소스/리소스_이름` : 특정 편집기를 사용하여 리소스 편집
- `kubectl edit 리소스/리소스_이름 -o json` : JSON 형식으로 리소스 편집

## Links
- More information: https://kubernetes.io/docs/reference/kubectl/generated/kubectl_edit/
