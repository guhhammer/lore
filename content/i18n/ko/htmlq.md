# htmlq

CSS 선택자를 사용하여 HTML 파일에서 원하는 내용을 추출하는 도구.

## Usage
- `cat 경로/대상/파일.html | htmlq '.card'` : `card` 클래스의 모든 요소 반환
- `cat 경로/대상/파일.html | htmlq -t 'p:first-of-type'` : 첫 번째 p 요소의 텍스트 내용 가져오기
- `cat 경로/대상/파일.html | htmlq -a href 'a'` : 페이지 내 모든 링크 추출
- `cat 경로/대상/파일.html | htmlq -r 'img' -r 'svg'` : 페이지에서 모든 이미지와 SVG 제거
- `htmlq -p -f 경로/대상/입력파일.html -o 경로/대상/출력파일.html` : 보기 좋게 포맷하여 파일로 출력

## Links
- More information: https://github.com/mgdm/htmlq#usage
