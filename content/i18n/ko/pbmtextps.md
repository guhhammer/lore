# pbmtextps

텍스트를 PostScript를 사용하여 PBM 이미지로 렌더링.

## Usage
- `pbmtextps "Hello World!" >경로/대상/출력.pbm` : 한 줄의 텍스트를 PBM 이미지로 렌더링
- `pbmtextps -font Times-Roman -fontsize 30 "Hello World!" >경로/대상/출력.pbm` : 폰트와 폰트 크기 지정
- `pbmtextps -l 70 -t 162 "Hello World!" >경로/대상/출력.pbm` : 원하는 좌측 및 상단 여백 지정
- `pbmtextps -du "Hello World!" >경로/대상/출력.ps` : 렌더링된 텍스트를 PBM 이미지로 출력하지 않고, 이 이미지를 생성할 PostScript 프로그램으로 출력

## Links
- More information: https://netpbm.sourceforge.net/doc/pbmtextps.html
