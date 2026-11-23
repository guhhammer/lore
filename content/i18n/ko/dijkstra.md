# dijkstra

그래프에서 단일 시작 노드로부터의 최단 경로 거리를 계산.

## Usage
- `dijkstra 소스_노드_파일` : 그래프 파일에서 지정한 시작 노드로부터의 거리를 계산
- `dijkstra -d 소스_노드_파일` : 거리를 계산할 때 그래프를 방향(directed) 그래프로 간주
- `dijkstra -p 소스_노드_파일` : 최단 경로에서 각 노드의 이전(previous) (가장 가까운) 노드를 기록
- `dijkstra -a 소스_노드_파일` : 도달할 수 없는 노드에 대해 큰 거리 값을 할당(assign)

## Links
- More information: https://graphviz.org/pdf/dijkstra.1.pdf
