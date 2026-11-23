# pamcut

Netpbm 이미지에서 직사각형 영역을 잘라내기.

## Usage
- `pamcut -cropl 값 -cropr 값 -cropt 값 -cropb 값 경로/대상/이미지.ppm >경로/대상/출력.ppm` : 이미지의 양쪽에서 지정된 열/행 수만큼 잘라내기
- `pamcut -l 값 -ri 값 경로/대상/이미지.ppm >경로/대상/출력.ppm` : 지정된 열 사이의 열만 유지하기 (포함)
- `pamcut -t 값 -b 값 -pad 경로/대상/이미지.ppm >경로/대상/출력.ppm` : 지정된 직사각형이 입력 이미지 내에 완전히 포함되지 않을 경우, 누락된 영역을 검은색 픽셀로 채우기

## Links
- More information: https://netpbm.sourceforge.net/doc/pamcut.html
