# tgatoppm

TrueVision Targa 파일을 Netpbm 이미지로 변환.

## Usage
- `tgatoppm 경로/대상/파일.tga >경로/대상/출력.ppm` : trueVision Targa 파일을 PPM 이미지로 변환
- `tgatoppm -h 경로/대상/파일.tga >경로/대상/출력.ppm` : TGA 헤더의 정보를 `stdout`로 덤프
- `tgatoppm -a 경로/대상/투명도_파일.pgm 경로/대상/파일.tga >경로/대상/출력.ppm` : 입력 이미지의 투명 채널 값을 지정한 파일에 작성
- `tgatoppm -v` : 버전 표시

## Links
- More information: https://netpbm.sourceforge.net/doc/tgatoppm.html
