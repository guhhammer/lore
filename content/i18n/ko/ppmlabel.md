# ppmlabel

PPM 이미지에 텍스트 추가.

## Usage
- `ppmlabel -x 위치_x -y 위치_y -t 텍스트 경로/대상/입력_파일.ppm >경로/대상/출력_파일.ppm` : 지정된 위치에 PPM 이미지에 텍스트 추가
- `ppmlabel -x 위치_x1 -y 위치_y1 -t 텍스트1 -x 위치_x2 -y 위치_y2 -t 텍스트2 경로/대상/입력_파일.ppm >경로/대상/출력_파일.ppm` : 서로 다른 위치에 여러 텍스트 추가
- `ppmlabel -x 위치_x -y 위치_y -c 선_색상 -b 배경_색상 -a 기울기 -s 크기 -t 텍스트 경로/대상/입력_파일.ppm >경로/대상/출력_파일.ppm` : 추가된 텍스트의 선 색상, 배경 색상, 기울기 및 크기 지정

## Links
- More information: https://netpbm.sourceforge.net/doc/ppmlabel.html
