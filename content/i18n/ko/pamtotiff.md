# pamtotiff

PAM 이미지를 TIFF 파일로 변환.

## Usage
- `pamtotiff 경로/대상/입력_파일.pam >경로/대상/출력_파일.tiff` : PAM 이미지를 TIFF 이미지로 변환
- `pamtotiff -none|packbits|lzw|g3|g4|flate|adobeflate 경로/대상/입력_파일.pam >경로/대상/출력_파일.tiff` : 출력 파일의 압축 방식을 명시적으로 지정
- `pamtotiff -c 경로/대상/입력_파일.pam >경로/대상/출력_파일.tiff` : 입력 이미지가 그레이스케일일지라도 항상 컬러 TIFF 이미지를 생성

## Links
- More information: https://netpbm.sourceforge.net/doc/pamtotiff.html
