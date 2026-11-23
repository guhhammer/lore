# sfdp

`graphviz` 파일에서 `스케일드 강제 지향` 네트워크 그래프의 이미지를 렌더링.

## Usage
- `sfdp -T png -O 경로/대상/입력.gv` : 입력 파일 이름과 출력 형식에 기반하여 파일 이름이 지정된 PNG 이미지 렌더링 (대문자 -O 사용)
- `sfdp -T svg -o 경로/대상/이미지.svg 경로/대상/입력.gv` : 지정된 출력 파일 이름으로 SVG 이미지 렌더링 (소문자 -o 사용)
- `sfdp -T 형식 -O 경로/대상/입력.gv` : PS, PDF, SVG, Fig, PNG, GIF, JPEG, JSON, 또는 DOT 형식으로 출력 렌더링
- `echo "digraph {this -> that} " | sfdp -T gif >경로/대상/이미지.gif` : `stdin`과 `stdout`을 사용하여 GIF 이미지 렌더링
- `sfdp -?` : 도움말 표시

## Links
- More information: https://graphviz.org/doc/info/command.html
