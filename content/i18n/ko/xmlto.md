# xmlto

XML 문서에 XSL 스타일시트를 적용.

## Usage
- `xmlto pdf 문서.xml` : docBook XML 문서를 PDF 형식으로 변환
- `xmlto -o 경로/대상/html_파일 html 문서.xml` : docBook XML 문서를 HTML 형식으로 변환하고 결과 파일을 별도의 디렉토리에 저장
- `xmlto html-nochunks 문서.xml` : docBook XML 문서를 단일 HTML 파일로 변환
- `xmlto -x 스타일시트.xsl 출력_포맷 문서.xml` : docBook XML 문서를 변환할 때 사용할 스타일시트 지정

## Links
- More information: https://manned.org/xmlto
