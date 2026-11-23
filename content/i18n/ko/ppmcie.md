# ppmcie

CIE 색상 차트를 PPM 이미지로 그리기.

## Usage
- `ppmcie >경로/대상/출력.ppm` : REC709 색상 시스템을 사용하여 CIE 색상 차트를 PPM 이미지로 그리기
- `ppmcie -cie|ebu|hdtv|ntsc|smpte >경로/대상/출력.ppm` : 사용할 색상 시스템 지정
- `ppmcie -red|green|blue xpos ypos >경로/대상/출력.ppm` : 개별 조명의 위치 지정
- `ppmcie -f >경로/대상/출력.ppm` : 맥스웰 삼각형 외부 영역을 흐리지 않음

## Links
- More information: https://netpbm.sourceforge.net/doc/ppmcie.html
