# chroma

범용 구문 강조 표시기. `--lexer` 옵션은 파일 확장자에 따라 자동으로 결정되므로, 일반적으로 필요하지 않음.

## Usage
- `chroma -l python 경로/대상/소스_파일.py` : python 어희 분석기를 사용해 파일에서 소스 코드를 강조 표시하고 `stdout`으로 출력
- `chroma -l go -f html 경로/대상/소스_파일.go >경로/대상/대상_파일.html` : go 어휘 분석기를 사용하여 파일의 소스코드를 강조 표시하고 HTML 파일로 출력
- `명령어 | chroma -l c++ -f svg -s monokai >경로/대상/대상_파일.svg` : C++ 어휘 분석기를 사용하여 `stdin`의 소스코드를 강조표시하고, Monokai 스타일을 사용하여 SVG 파일로 출력
- `chroma --list` : 사용 가능한 어휘 분석기, 스타일 및 포맷터 목록 나열

## Links
- More information: https://manned.org/chroma
