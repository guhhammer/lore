# pbmtoepsi

PBM 이미지를 캡슐화된 PostScript 스타일의 미리보기 비트맵으로 변환.

## Usage
- `pbmtoepsi 경로/대상/이미지.pbm >경로/대상/출력.bmp` : PBM 이미지를 캡슐화된 PostScript 스타일의 미리보기 비트맵으로 변환
- `pbmtoepsi -d 144 경로/대상/이미지.pbm >경로/대상/출력.bmp` : 지정한 해상도로 정사각형 출력 이미지 생성
- `pbmtoepsi -d 72x144 경로/대상/이미지.pbm >경로/대상/출력.bmp` : 지정한 가로 및 세로 해상도로 출력 이미지 생성
- `pbmtoepsi -b 경로/대상/이미지.pbm >경로/대상/출력.bmp` : 경계 상자만 생성

## Links
- More information: https://netpbm.sourceforge.net/doc/pbmtoepsi.html
