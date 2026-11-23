# textutil

다양한 형식의 텍스트 파일을 조작.

## Usage
- `textutil -info 경로/대상/foo.rtf` : `foo.rtf` 파일에 대한 정보 표시
- `textutil -convert html 경로/대상/foo.rtf` : `foo.rtf`를 `foo.html`로 변환
- `textutil 경로/대상/foo.rtf -convert txt` : 서식 있는 텍스트를 일반 텍스트로 변환
- `textutil -convert rtf -font Times -fontsize 10 경로/대상/foo.txt` : `foo.txt`를 `foo.rtf`로 변환, Times 10 폰트 사용
- `textutil -cat html -title "Several Files" -output 경로/대상/index.html *.rtf` : 현재 디렉터리의 모든 RTF 파일을 불러와 내용을 연결하고, 결과를 `index.html`로 작성하며 HTML 제목을 "Several Files"로 설정

## Links
- More information: https://keith.github.io/xcode-man-pages/textutil.1.html
