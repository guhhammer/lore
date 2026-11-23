# tidy

HTML, XHTML 및 XML 파일을 정리하고 보기 좋게 출력.

## Usage
- `tidy 경로/대상/파일.html` : HTML 파일을 보기 좋게 출력
- `tidy -i y -w 100 -o 경로/대상/output.html 경로/대상/파일.html` : indentation을 활성화하고, 줄을 100으로 wrapping하여 `output.html`에 저장
- `tidy -config 경로/대상/설정 -m 경로/대상/파일.html` : 설정 파일을 사용하여 HTML 파일을 직접 수정
