# elasticsearch-node

노드 종료, 재구성, 정보 조회 등 Elasticsearch 노드의 저수준 작업을 관리하는 도구.

## Usage
- `elasticsearch-node info` : 현재 노드 정보 출력
- `elasticsearch-node unsafe-bootstrap` : 전체 클러스터 재시작을 위한 노드 준비 (예: 업그레이드 후)
- `elasticsearch-node repurpose` : 노드 역할 변경을 위한 재구성 (예: master에서 data 노드)
- `elasticsearch-node roles` : 노드에 할당된 역할 목록 출력
- `elasticsearch-node diagnostics` : JVM 버전, Elasticsearch 홈 경로 등 진단 정보 출력
- `elasticsearch-node -h` : 도움말 표시
