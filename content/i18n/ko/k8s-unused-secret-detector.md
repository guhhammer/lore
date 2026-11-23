# k8s-unused-secret-detector

사용되지 않는 Kubernetes 시크릿 감지.

## Usage
- `k8s-unused-secret-detector` : 사용되지 않는 시크릿 감지
- `k8s-unused-secret-detector -n 네임스페이스` : 특정 네임스페이스에서 사용되지 않는 시크릿 감지
- `k8s-unused-secret-detector -n 네임스페이스 | kubectl delete secrets -n 네임스페이스` : 특정 네임스페이스에서 사용되지 않는 시크릿 삭제

## Links
- More information: https://github.com/dtan4/k8s-unused-secret-detector
