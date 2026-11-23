# pnmtofiasco

PNM 이미지를 압축된 FIASCO 파일로 변환.

## Usage
- `pnmtofiasco 경로/대상/파일.pnm >경로/대상/파일.fiasco` : PNM 이미지를 압축된 FIASCO 파일로 변환
- `pnmtofiasco -i "img[01-09+1].pnm" >경로/대상/파일.fiasco` : 패턴을 통해 input 파일 지정
- `pnmtofiasco -q 품질_수준 경로/대상/파일.pnm >경로/대상/파일.fiasco` : 압축 품질 지정
- `pnmtofiasco -f 경로/대상/fiascorc 경로/대상/파일.pnm >경로/대상/파일.fiasco` : 지정된 구성 파일에서 사용할 옵션 로드

## Links
- More information: https://netpbm.sourceforge.net/doc/pnmtofiasco.html
