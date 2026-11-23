# kubectl auth

Kubernetes 클러스터의 접근 권한 확인.

## Usage
- `kubectl auth can-i '*' '*' -n 네임스페이스` : 특정 네임스페이스에서 현재 사용자가 모든 리소스에 대해 모든 작업을 수행할 수 있는지 확인
- `kubectl auth can-i verb 리소스 -n 네임스페이스` : 특정 네임스페이스에서 현재 사용자가 특정 리소스에 대해 특정 작업을 수행할 수 있는지 확인
- `kubectl auth can-i verb 리소스 -n 네임스페이스 --as user_or_sa` : 특정 사용자 또는 서비스 계정이 특정 리소스에 대해 작업을 수행할 수 있는지 확인
- `kubectl auth can-i --list -n 네임스페이스` : 특정 네임스페이스에서 현재 사용자가 수행 가능한 모든 작업 목록 표시

## Links
- More information: https://kubernetes.io/docs/reference/kubectl/generated/kubectl_auth/
