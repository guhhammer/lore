# pamfix

PAM, PBM, PGM 및 PPM 파일의 오류를 수정합니다.

## Usage
- `pamfix -t 경로/대상/손상된_파일.ext >경로/대상/출력_파일.ext` : 마지막 부분이 손상된 Netpbm 파일 수정
- `pamfix -cl 경로/대상/손상된_파일.ext >경로/대상/출력_파일.ext` : 이미지의 `maxval`을 초과하는 픽셀 값을 낮추어 수정
- `pamfix -ch 경로/대상/손상된.pam|pbm|pgm|ppm >경로/대상/출력.pam|pbm|pgm|ppm` : 이미지의 `maxval`을 초과하는 픽셀 값을 증가시켜 수정

## Links
- More information: https://netpbm.sourceforge.net/doc/pamfix.html
