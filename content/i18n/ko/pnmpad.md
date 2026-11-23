# pnmpad

PNM 이미지에 테두리를 추가.

## Usage
- `pnmpad -l 100 -ri 150 -t 123 -bo 456 경로/대상/이미지.pnm >경로/대상/출력.pnm` : 이미지에 지정된 크기의 테두리 추가
- `pnmpad -wi 1000 -he 500 경로/대상/이미지.pnm >경로/대상/출력.pnm` : 이미지를 지정된 크기로 패딩
- `pnmpad -wi 1000 -ha 0.7 경로/대상/이미지.pnm >경로/대상/출력.pnm` : 이미지의 너비를 지정된 크기로 패딩하고, 좌우 패딩 비율 조정
- `pnmpad -wi 1000 -c red 경로/대상/이미지.pnm >경로/대상/출력.pnm` : 지정된 색상을 사용하여 이미지의 너비를 패딩

## Links
- More information: https://netpbm.sourceforge.net/doc/pnmpad.html
