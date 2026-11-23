# gvgen

단순하고 구조화된 추상 그래프 생성.

## Usage
- `gvgen -c 10` : 정점과 엣지가 10개인 사이클(cycle) 그래프 생성 후 `stdout`에 출력
- `gvgen -g 4,3` : 4×3 그리드(grid)
- `gvgen -t 5` : 높이 5의 이진트리(tree) 생성
- `gvgen -b 3,4` : 정점이 각각 3개와 4개인 완전 이분 그래프(bipartite graph)
- `gvgen -r 10,5 -o random.gv` : 랜덤(random) 그래프 생성 후 파일로 출력(output)
- `gvgen -d -v -c 6` : 방향(directed) 그래프를 자세한(verbose) 출력과 함께 생성
- `gvgen -?` : 도움말 표시

## Links
- More information: https://graphviz.org/pdf/gvgen.1.pdf
