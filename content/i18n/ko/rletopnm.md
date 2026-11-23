# rletopnm

Utah Raster Tools RLE 이미지 파일을 PNM 파일로 변환.

## Usage
- `rletopnm 경로/대상/입력.rle >경로/대상/출력.pnm` : RLE 이미지를 PNM 파일로 변환
- `rletopnm --a 경로/대상/알파_파일.pgm 경로/대상/입력.rle >경로/대상/출력.pnm` : RLE 파일의 알파 채널을 포함하는 PGM 이미지 생성
- `rletopnm --verb 경로/대상/입력.rle >경로/대상/출력.pnm` : 자세한 모드로 작동하고 RLE 헤더의 내용을 `stdout`에 출력

## Links
- More information: https://netpbm.sourceforge.net/doc/rletopnm.html
