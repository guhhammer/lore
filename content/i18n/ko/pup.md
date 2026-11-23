# pup

명령줄 HTML 파싱 도구.

## Usage
- `cat index.html | pup --color` : 원시 HTML 파일을 정리되고 들여쓰기된 색상 형식으로 변환
- `cat index.html | pup '태그'` : 요소 태그 이름으로 HTML 필터링
- `cat index.html | pup 'div#아이디'` : ID로 HTML 필터링
- `cat index.html | pup 'input[type="text"]'` : 속성 값으로 HTML 필터링
- `cat index.html | pup 'div text{}'` : 필터링된 HTML 요소와 그 자식 요소의 모든 텍스트 출력
- `cat index.html | pup 'div json{}'` : HTML을 JSON으로 출력

## Links
- More information: https://github.com/ericchiang/pup
