# highlight

구문 강조된 소스 코드를 다양한 형식으로 출력. 더 많은 정보: <http://andre-simon.de/doku/highlight/en/highlight.php>.

## Usage
- `highlight -O html -s 테마_이름 -S 언어 경로/대상/소스_코드` : 소스 코드 파일에서 완전한 HTML 문서를 생성
- `highlight -O html -f -S 언어 소스_파일` : 더 큰 문서에 포함하기에 적합한 HTML 조각을 생성
- `highlight -O html --inline-css -S 언어 소스_파일` : 모든 태그에 CSS 스타일을 인라인
- `highlight --list-scripts langs|themes|plugins` : 지원되는 모든 언어, 테마 또는 플러그인을 나열
- `highlight -O html --print-style -s 테마_이름 -S 언어 --stdout` : 테마에 대한 CSS 스타일시트를 출력
