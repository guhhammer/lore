# k3s

경량 Kubernetes 클러스터 설치 및 관리.

## Usage
- `k3s kubectl get nodes` : 내장된 `kubectl` 명령 실행
- `k3s etcd-snapshot save` : 클러스터의 etcd 스냅샷 생성
- `k3s certificate rotate-ca` : CA 인증서 교체 수행
- `k3s token list` : 부트스트랩 토큰 관리
- `k3s-uninstall.sh` : K3s 및 모든 구성 요소 제거

## Links
- More information: https://docs.k3s.io/cli
