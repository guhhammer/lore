# kubectl patch

Kubernetes 리소스의 일부 값을 부분적으로 수정.

## Usage
- `kubectl patch secrets secret_이름 -p '{"metadata":{"finalizers": []\}\}' --type merge` : strategic Merge JSON Patch를 사용하여 Secret의 finalizer 제거
- `kubectl patch secrets secret_이름 -p $'metadata:\n finalizers: []' --type merge` : strategic Merge YAML Patch를 사용하여 Secret의 finalizer 제거
- `kubectl patch po pod_이름 --type 'json' -p '[{"op": "replace", "path": "/spec/containers/0/image", "value":"새로운_이미지_값"}]'` : 위치 기반 배열(positional arrays)을 사용하는 JSON Patch로 Pod의 컨테이너를 부분 수정
- `kubectl patch deploy deployment_이름 --subresource 'scale' --type 'merge' -p '{"spec":{"replicas":replica_개수\}\}'` : scale 서브리소스를 통해 Deployment의 Replica 수를 변경

## Links
- More information: https://kubernetes.io/docs/reference/kubectl/generated/kubectl_patch/
