# loft

가상 클러스터를 사용하여 멀티테넌트 Kubernetes 환경 설치 및 관리.

## Usage
- `loft start` : 현재 Kubernetes 클러스터에 Loft 설치 또는 업그레이드
- `loft login https://loft.example.com` : 원격 Loft 인스턴스에 로그인
- `loft create vcluster 가상클러스터_이름 -s 스페이스_이름 -c 클러스터_이름` : 특정 스페이스와 클러스터에 가상 클러스터 생성
- `loft list vclusters` : 모든 가상 클러스터 목록 조회
- `loft use vcluster 가상클러스터_이름` : 특정 가상 클러스터로 컨텍스트 전환
- `loft delete vcluster 가상클러스터_이름` : 가상 클러스터 삭제
- `loft vars username` : 현재 Loft 사용자 이름을 확인
- `loft uninstall` : 클러스터에서 Loft 제거

## Links
- More information: https://loft.sh/docs/cli/loft/
