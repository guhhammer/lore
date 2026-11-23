# palmtopnm

Palm 비트맵 파일을 PNM 이미지로 변환.

## Usage
- `palmtopnm 경로/대상/파일.palm >경로/대상/파일.pnm` : palm 비트맵을 PNM 이미지로 변환
- `palmtopnm -verb 경로/대상/파일.palm >경로/대상/파일.pnm` : 입력 파일에 대한 정보 표시
- `palmtopnm -r n 경로/대상/파일.palm >경로/대상/파일.pnm` : 입력 파일에 포함된 이미지의 n번째 렌디션 변환
- `palmtopnm -s 경로/대상/파일.palm >경로/대상/파일.pnm` : 입력 파일의 색상 히스토그램을 `stdout`에 출력
- `palmtopnm -t 경로/대상/파일.palm` : 설정된 경우 입력 이미지의 투명 색상 출력

## Links
- More information: https://netpbm.sourceforge.net/doc/palmtopnm.html
