# kops

Kubernetes 클러스터를 생성, 삭제, 업그레이드 및 유지 관리.

## Usage
- `kops create cluster -f 클러스터_이름.yaml` : 구성 사양에서 클러스터 생성
- `kops create sshpublickey 키_이름 -i ~/.ssh/id_rsa.pub` : 새로운 SSH 공개 키 생성
- `kops export kubecfg 클러스터_이름` : 클러스터 구성을 `~/.kube/config` 파일로 내보내기
- `kops get cluster 클러스터_이름 -o yaml` : 클러스터 구성을 YAML로 가져오기
- `kops delete cluster 클러스터_이름 -y` : 클러스터 삭제
- `kops validate cluster 클러스터_이름 --wait 준비_시간 --count 필요한_검증_수` : 클러스터 유효성 검사

## Links
- More information: https://kops.sigs.k8s.io/cli/kops/
