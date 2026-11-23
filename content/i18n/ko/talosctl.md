# talosctl

최소 구성 및 불변 방식의 Kubernetes 배포판인 Talos Linux와 상호작용.

## Usage
- `talosctl apply-config -i -n 컨트롤_플레인_ip -f 경로/대상/컨트롤_플레인.yaml` : 새로운 노드에 설정을 적용
- `talosctl bootstrap -n node_ip` : 노드에서 `etcd` 클러스터 부트스트랩
- `talosctl edit 편집할_리소스 -n 노드_ip` : API 리소스 편집
- `talosctl get 가져올_리소스 -n 노드_ip` : 리소스 조회
- `talosctl kubeconfig -n 노드_ip` : 노드에서 관리자용 kube 구성 다운로드
- `talosctl reset -n 노드_ip` : 노드 초기화

## Links
- More information: https://docs.siderolabs.com/talos/latest/reference/cli
