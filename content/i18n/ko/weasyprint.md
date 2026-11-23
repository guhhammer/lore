# weasyprint

HTML을 PDF 또는 PNG로 렌더링.

## Usage
- `weasyprint 경로/대상/입력.html 경로/대상/출력.pdf` : HTML 파일을 PDF로 렌더링
- `weasyprint 경로/대상/입력.html 경로/대상/출력.png --stylesheet 경로/대상/스타일시트.css` : 추가 사용자 스타일시트를 포함하여 HTML 파일을 PNG로 렌더링
- `weasyprint 경로/대상/입력.html 경로/대상/출력.pdf --verbose` : 렌더링 시 추가 디버깅 정보 출력
- `weasyprint 경로/대상/입력.html 경로/대상/출력.png --resolution 300` : PNG로 출력할 때 사용자 지정 해상도 지정
- `weasyprint 경로/대상/입력.html 경로/대상/출력.png --base-url url_또는_파일_이름` : 입력 HTML 파일의 상대 URL에 대한 기본 URL 지정
