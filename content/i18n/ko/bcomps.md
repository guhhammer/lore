# bcomps

그래프를 이중 연결 구성 요소로 분해.

## Usage
- `bcomps 경로/대상/input1.gv 경로/대상/input2.gv ... >경로/대상/output.gv` : 하나 이상의 그래프를 이중 연결 구성요소로 분해
- `bcomps -v -s 경로/대상/input1.gv 경로/대상/input2.gv ...` : 하나 이상의 그래프에서 블록 및 절단되는 정점 수를 인쇄
- `bcomps -x -o 경로/대상/output.gv 경로/대상/input1.gv 경로/대상/input2.gv ...` : `output.gv`를 기반으로 각 블록과 블록-절단정점 트리를 여러 번호의 파일 이름에 쓰기
- `bcomps -?` : 도움말 표시

## Links
- More information: https://graphviz.org/pdf/bcomps.1.pdf
