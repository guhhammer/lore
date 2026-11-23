# unflatten

방향 그래프의 레이아웃 가로 세로 비율을 개선하기 위해 조정.

## Usage
- `unflatten 경로/대상/input1.gv 경로/대상/input2.gv ... >경로/대상/output.gv` : 하나 이상의 방향 그래프를 조정하여 레이아웃 가로 세로 비율을 개선
- `unflatten 경로/대상/입력.gv | dot -T png 경로/대상/출력.png` : `unflatten`을 `dot` 레이아웃 전처리기로 사용하여 가로 세로 비율 개선
- `unflatten -?` : 도움말 표시

## Links
- More information: https://www.graphviz.org/pdf/unflatten.1.pdf
