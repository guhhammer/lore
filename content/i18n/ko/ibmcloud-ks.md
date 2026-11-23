# ibmcloud ks

IBM Cloud의 Kubernetes 및 OpenShift 클러스터 관리.

## Usage
- `ibmcloud ks cluster get -c 클러스터_아이디` : 클러스터 상세 정보 표시
- `ibmcloud ks cluster ca status -c 클러스터_아이디` : 클러스터의 인증 기관 인증서 교체 상태 확인
- `ibmcloud ks worker-pool ls -c 클러스터_아이디` : 클러스터의 워커 풀 목록 표시
- `ibmcloud ks worker replace -c 클러스터_아이디 -w worker_id` : 워커 노드를 삭제하고 동일한 워커 풀에 새 노드로 교체
- `ibmcloud ks help` : 이 명령에서 사용 가능한 모든 작업 목록 표시
