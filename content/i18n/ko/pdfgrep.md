# pdfgrep

PDF 파일에서 텍스트 검색.

## Usage
- `pdfgrep 패턴 파일.pdf` : PDF에서 패턴과 일치하는 줄 찾기
- `pdfgrep -H -n 패턴 파일.pdf` : 각 일치하는 줄에 대해 파일 이름과 페이지 번호 포함
- `pdfgrep -m 3 -i '^foo' 파일.pdf` : "foo"로 시작하는 줄을 대소문자 구분 없이 검색하고 처음 3개의 일치 항목 반환
- `pdfgrep -r 패턴` : 현재 디렉토리에서 `.pdf` 확장자를 가진 파일을 재귀적으로 검색하여 패턴 찾기
- `pdfgrep -r --include '*book.pdf' 패턴` : 현재 디렉토리에서 특정 글롭과 일치하는 파일을 재귀적으로 검색하여 패턴 찾기

## Links
- More information: https://pdfgrep.org/doc.html
