# tifftopnm

TIFF 이미지를 PNM 이미지로 변환.

## Usage
- `tifftopnm 경로/대상/입력_파일.tiff >경로/대상/출력_파일.pnm` : TIFF를 PNM 파일로 변환
- `tifftopnm -a 경로/대상/알파_파일.pgm 경로/대상/입력_파일.tiff >경로/대상/출력_파일.pnm` : 입력 이미지의 알파 채널을 포함하는 PGM 파일 생성
- `tifftopnm -r 경로/대상/입력_파일.tiff >경로/대상/출력_파일.pnm` : 입력 TIFF 이미지의 `fillorder` 태그 고려
- `tifftopnm -h 경로/대상/입력_파일.tiff >경로/대상/출력_파일.pnm` : TIFF 헤더 정보를 `stderr`에 출력

## Links
- More information: https://netpbm.sourceforge.net/doc/tifftopnm.html
