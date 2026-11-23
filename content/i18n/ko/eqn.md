# eqn

groff (GNU Troff) 문서 형식화 시스템용 방정식 전처리.

## Usage
- `eqn 경로/대상/입력.eqn >경로/대상/출력.roff` : 방정식으로 입력을 처리하고, groff를 사용하여 향후 조판을 위해 출력을 PostScript에 저장
- `eqn -T pdf 경로/대상/입력.eqn | groff -me -T pdf >경로/대상/출력.pdf` : 매크로([me] macro) 패키지를 사용하여 방정식이 포함된 입력 파일을 PDF로 조판

## Links
- More information: https://manned.org/eqn
