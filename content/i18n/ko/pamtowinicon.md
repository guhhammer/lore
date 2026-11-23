# pamtowinicon

PAM 이미지를 Windows ICO 파일로 변환.

## Usage
- `pamtowinicon 경로/대상/입력_파일.pam >경로/대상/출력.ico` : PAM 이미지 파일을 ICO 파일로 변환
- `pamtowinicon -pn t 경로/대상/입력_파일.pam >경로/대상/출력.ico` : 해상도가 `t`보다 작은 이미지는 BMP 형식으로, 그 외 이미지는 PNG 형식으로 인코딩
- `pamtowinicon -t 경로/대상/입력_파일.pam >경로/대상/출력.ico` : 불투명하지 않은 영역 외의 모든 픽셀을 검정색으로 설정

## Links
- More information: https://netpbm.sourceforge.net/doc/pamtowinicon.html
