# ccomps

그래프를 연결된 구성 요소로 분해.

## Usage
- `ccomps 경로/대상/입력파일1.gv 경로/대상/입력파일2.gv ... >경로/대상/출력파일.gv` : 하나 이상의 그래프를 연결된 구성 요소로 분해
- `ccomps -v -s 경로/대상/입력파일1.gv 경로/대상/입력파일2.gv ...` : 하나 이상의 그래프에서 노드, 간선 및 연결된 구성요소의 수를 인쇄
- `ccomps -x -o 경로/대상/출력파일.gv 경로/대상/입력파일1.gv 경로/대상/입력파일2.gv ...` : `output.gv`를 기반으로 번호가 매겨진 파일이름에 연결된 각 구성요소를 작성
- `ccomps -?` : 도움말 표시

## Links
- More information: https://graphviz.org/pdf/ccomps.1.pdf
