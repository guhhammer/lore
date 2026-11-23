# pgmtoppm

PGM 이미지를 색상화.

## Usage
- `pgmtoppm -b red -w blue 경로/대상/입력.pgm >경로/대상/출력.ppm` : 입력 이미지의 모든 회색조 값을 두 가지 지정된 색상 사이의 모든 색상으로 매핑
- `pgmtoppm -m 경로/대상/색상표.ppm 경로/대상/입력.pgm >경로/대상/출력.ppm` : 입력 이미지의 모든 회색조 값을 지정된 색상표에 따라 색상으로 매핑

## Links
- More information: https://netpbm.sourceforge.net/doc/pgmtoppm.html
