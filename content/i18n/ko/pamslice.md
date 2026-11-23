# pamslice

PAM 이미지에서 한 줄의 값을 추출.

## Usage
- `pamslice -r n 경로/대상/이미지.pam` : n번째 행의 픽셀 값을 테이블 형식으로 출력
- `pamslice -c n 경로/대상/이미지.pam` : n번째 열의 픽셀 값을 테이블 형식으로 출력
- `pamslice -r n -plane m 경로/대상/이미지.pam` : 입력 이미지의 m번째 평면만 고려
- `pamslice -r n -x 경로/대상/이미지.pam` : 시각화를 위한 `xmgr` 입력 형식으로 출력 생성

## Links
- More information: https://netpbm.sourceforge.net/doc/pamslice.html
