# pbmtoescp2

PBM 이미지를 ESC/P2 프린터 파일로 변환.

## Usage
- `pbmtoescp2 경로/대상/이미지.pbm >경로/대상/출력.escp2` : PBM 이미지를 ESC/P2 프린터 파일로 변환
- `pbmtoescp2 -c 0|1 경로/대상/이미지.pbm >경로/대상/출력.escp2` : 출력의 압축 지정
- `pbmtoescp2 -re 180|360|720 경로/대상/이미지.pbm >경로/대상/출력.escp2` : 출력을 인치당 도트 수로 가로 및 세로 해상도 지정
- `pbmtoescp2 -f 경로/대상/이미지.pbm >경로/대상/출력.escp2` : 출력의 끝에 폼피드 명령 추가

## Links
- More information: https://netpbm.sourceforge.net/doc/pbmtoescp2.html
