# ppmtomitsu

PPM 이미지를 Mitsubishi S340-10 파일로 변환.

## Usage
- `ppmtomitsu 경로/대상/파일.ppm >경로/대상/파일.mitsu` : PPM 이미지를 MITSU 파일로 변환
- `ppmtomitsu -e 1|2|3 -s 1|2|3|4 -c n 경로/대상/파일.ppm >경로/대상/파일.mitsu` : 이미지를 지정된 배율로 확대하고, 지정된 선명도를 사용하여 `n`개의 복사본 생성
- `ppmtomitsu -m A|A4|AS|A4S 경로/대상/파일.ppm >경로/대상/파일.mitsu` : 인쇄 과정에 주어진 매체 사용

## Links
- More information: https://netpbm.sourceforge.net/doc/ppmtomitsu.html
