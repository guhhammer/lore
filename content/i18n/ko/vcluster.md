# vcluster

네임스페이스 내에서 경량 가상 Kubernetes 클러스터를 생성하고 관리.

## Usage
- `vcluster create vcluster_이름 -n 네임스페이스` : 지정한 네임스페이스에 가상 클러스터 생성
- `vcluster connect vcluster_이름 -n 네임스페이스 --local-port port --insecure` : 로컬 포트와 Insecure 모드를 사용하여 가상 클러스터에 연결
- `vcluster list` : 모든 가상 클러스터 목록 표시
- `vcluster delete vcluster_이름` : 가상 클러스터 삭제
- `vcluster platform list` : 플랫폼에서 관리하는 가상 클러스터 목록 표시
- `vcluster platform create vcluster_이름 -n 네임스페이스` : 플랫폼에서 관리하는 가상 클러스터 생성
- `vcluster platform connect vcluster_이름 -n 네임스페이스` : 플랫폼에서 관리하는 가상 클러스터에 연결
- `vcluster platform delete vcluster_이름 -n 네임스페이스` : 플랫폼에서 관리하는 가상 클러스터 삭제

## Links
- More information: https://www.vcluster.com/docs/vcluster
