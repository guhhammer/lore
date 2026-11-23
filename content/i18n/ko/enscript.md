# enscript

텍스트 파일을 PostScript, HTML, RTF, ANSI 및 겹쳐쓰기로 변환.

## Usage
- `enscript 경로/대상/입력_파일 -o 경로/대상/출력_파일` : 텍스트 파일에서 PostScript 파일
- `enscript 경로/대상/입력_파일 -w html|rtf|... -o 경로/대상/출력_파일` : postScript와 다른 언어로 파일을 생성
- `enscript 경로/대상/입력_파일 --columns 숫자 -r -o 경로/대상/출력_파일` : 페이지를 열(최대 9개)로 분할하여 가로 레이아웃으로 PostScript 파일을 생성
- `enscript --help-highlight` : 사용 가능한 구문 강조 언어 및 파일 형식 표시
- `enscript 경로/대상/입력_파일 --color 1 -E 언어 -o 경로/대상/출력_파일` : 지정된 언어에 대한 구문 강조 및 색상을 사용하여 PostScript 파일을 생성

## Links
- More information: https://manned.org/enscript
