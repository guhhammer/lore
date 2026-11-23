# pnmgamma

PNM 이미지에 감마 보정 수행.

## Usage
- `pnmgamma -bt709tolinear|bt709tosrgb 경로/대상/이미지.pnm >경로/대상/출력.pnm` : 이미지를 BT.709 휘도에서 방사선 또는 sRGB 휘도로 변환
- `pnmgamma -lineartobt709|srgbtobt709 경로/대상/이미지.pnm >경로/대상/출력.pnm` : 이미지를 방사선 또는 sRGB 휘도에서 BT.709 휘도로 변환
- `pnmgamma -ga 값 경로/대상/이미지.pnm >경로/대상/출력.pnm` : 감마 전송 함수에 사용될 감마 값 지정
- `pnmgamma -rg 값 -gg 값 -bg 값 경로/대상/이미지.pnm >경로/대상/출력.pnm` : 색 구성 요소별로 감마 전송 함수에 사용될 감마 값 지정

## Links
- More information: https://netpbm.sourceforge.net/doc/pnmgamma.html
