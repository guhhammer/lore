# pdfjam

LaTeX의 pdfpages 패키지를 사용하여 PDF를 처리하는 셸 프론트엔드.

## Usage
- `pdfjam 경로/대상/파일1.pdf 경로/대상/파일2.pdf ... -o 경로/대상/출력_파일.pdf` : 두 개 이상의 PDF 병합
- `pdfjam 경로/대상/파일1.pdf 1 경로/대상/파일2.pdf 1 ... -o 경로/대상/출력_파일.pdf` : 각 파일의 첫 페이지를 함께 병합
- `pdfjam 경로/대상/파일1.pdf 3-5,1 경로/대상/파일2.pdf 4-6 -o 경로/대상/출력_파일.pdf` : 두 PDF의 하위 범위 병합
- `pdfjam 경로/대상/파일.pdf 경로/대상/서명 --fitpaper true -o 경로/대상/서명된.pdf --nup "1x2" --delta "0 -842pt"` : 스캔된 서명을 오버레이하여 A4 페이지에 서명 (다른 형식의 경우 델타를 높이에 맞춤)
- `pdfjam 경로/대상/파일.pdf --nup 2x2 --suffix 4up --preamble '\usepackage{fancyhdr} \pagestyle{fancy}'` : 입력 파일의 페이지를 멋진 2x2 그리드로 배열
- `pdfjam 경로/대상/파일1.pdf last-1 경로/대상/파일2.pdf last-1 ... --suffix reversed` : 각 파일 내 페이지 순서를 반대로 하고 연결

## Links
- More information: https://github.com/pdfjam/pdfjam/blob/master/doc/pdfjam-help.txt
