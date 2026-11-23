# spottopgm

SPOT 위성 이미지를 PGM 형식으로 변환.

## Usage
- `spottopgm 경로/대상/파일.spot >경로/대상/출력파일.pgm` : 지정한 SPOT 이미지를 PGM 형식으로 변환
- `spottopgm -1|2|3 경로/대상/파일.spot >경로/대상/출력파일.pgm` : 지정한 색상 채널 추출
- `spottopgm first_col first_row last_col last_row 경로/대상/파일.spot >경로/대상/출력파일.pgm` : 입력 이미지에서 지정한 사각형 영역 추출

## Links
- More information: https://netpbm.sourceforge.net/doc/spottopgm.html
