# gvcolor

다양한 색상으로 순위가 매겨진 이중 그래프를 색깔 입히기.

## Usage
- `gvcolor 경로/대상/레이아웃1.gv 경로/대상/레이아웃2.gv ... >경로/대상/출력.gv` : 하나 이상의 순위가 매겨진 이중 그래프(이미 `dot`으로 처리됨)에 색상을 지정
- `dot 경로/대상/입력.gv | gvcolor | dot -T png >경로/대상/출력.png` : 그래프를 배치하고 색상을 지정한 다음, PNG 이미지로 변환
- `gvcolor -?` : 도움말 표시

## Links
- More information: https://graphviz.org/pdf/gvcolor.1.pdf
