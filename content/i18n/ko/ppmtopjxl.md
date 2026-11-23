# ppmtopjxl

PPM 이미지를 HP PaintJet XL PCL 파일로 변환.

## Usage
- `ppmtopjxl 경로/대상/이미지.ppm >경로/대상/출력파일.pjxl` : PPM 이미지를 PJXL 파일로 변환
- `ppmtopjxl -xsi 10cm -ysi 5cm 경로/대상/이미지.ppm >경로/대상/출력파일.pjxl` : 입력 이미지 크기 조정
- `ppmtopjxl -xsh 10pt -ysh 5pt 경로/대상/이미지.ppm >경로/대상/출력파일.pjxl` : 입력 이미지 위치 이동
- `ppmtopjxl -n 경로/대상/이미지.ppm >경로/대상/출력파일.pjxl` : 기본 TIFF 4.0 압축 방식을 사용하지 않고 변환

## Links
- More information: https://netpbm.sourceforge.net/doc/ppmtopjxl.html
