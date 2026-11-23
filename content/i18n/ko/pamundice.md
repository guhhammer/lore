# pamundice

여러 개의 Netpbm 이미지를 하나로 결합.

## Usage
- `pamundice 파일이름_%1d_%1a.ppm -a 그리드_너비 -d 그리드_높이 >경로/대상/출력.ppm` : `printf` 스타일의 파일 이름 표현과 일치하는 이미지 결합. 특정 크기의 그리드를 가정
- `pamundice 파일이름_%1d_%1a.ppm -a x_값 -d y_값 -ho 값 -vo 값 >경로/대상/출력.ppm` : 타일이 지정된 양만큼 수평 및 수직으로 겹친다고 가정
- `pamundice -l 경로/대상/파일.txt -a x_값 -d y_값 >경로/대상/출력.ppm` : 결합할 이미지를 한 줄에 하나의 파일 이름이 있는 텍스트 파일로 지정

## Links
- More information: https://netpbm.sourceforge.net/doc/pamundice.html
