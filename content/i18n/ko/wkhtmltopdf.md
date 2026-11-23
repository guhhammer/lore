# wkhtmltopdf

HTML 문서나 웹 페이지를 PDF 파일로 변환하는 오픈 소스 명령줄 도구.

## Usage
- `wkhtmltopdf 입력.html 출력.pdf` : HTML 문서를 PDF로 변환
- `wkhtmltopdf --page-size A4 입력.html 출력.pdf` : PDF 페이지 크기 지정 (`QPrinter`의 `PaperSize`에서 지원되는 크기를 참조)
- `wkhtmltopdf --margin-top|bottom|left|right 10mm 입력.html 출력.pdf` : PDF 페이지 여백 설정
- `wkhtmltopdf --orientation Landscape|Portrait 입력.html 출력.pdf` : PDF 페이지 방향 설정
- `wkhtmltopdf --grayscale 입력.html 출력.pdf` : PDF 문서를 그레이스케일로 생성

## Links
- More information: https://wkhtmltopdf.org/usage/wkhtmltopdf.txt
