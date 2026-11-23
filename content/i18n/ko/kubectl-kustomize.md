# kubectl kustomize

`kustomization.yaml` 파일을 사용하여 Kubernetes 리소스 생성.

## Usage
- `kubectl kustomize` : 현재 디렉터리의 Kubernetes 리소스 빌드
- `kubectl kustomize 경로/대상/디렉터리` : 특정 디렉터리의 Kubernetes 리소스 빌드
- `kubectl kustomize https://github.com/user/repo/path` : 원격 URL의 Kubernetes 리소스 빌드
- `kubectl kustomize 경로/대상/디렉터리 >output.yaml` : 생성된 리소스를 파일로 저장
- `kubectl kustomize --load-restrictor LoadRestrictionsNone 경로/대상/디렉터리` : load restrictor를 비활성화하고 빌드 수행

## Links
- More information: https://kubernetes.io/docs/reference/kubectl/generated/kubectl_kustomize/
