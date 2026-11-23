# pt

Platinum Searcher. `ag`와 유사한 코드 검색 도구.

## Usage
- `pt foo` : "foo"를 포함하는 파일을 찾아 일치하는 부분을 강조하여 출력
- `pt -c foo` : "foo"를 포함하는 파일을 찾아 각 파일의 일치 개수 표시
- `pt -wi foo` : "foo"를 단어 전체로 취급하고 대소문자 무시하여 찾기
- `pt -G='\.bar$' foo` : 정규 표현식을 사용하여 특정 확장자를 가진 파일에서 "foo" 찾기
- `pt --depth=2 -e '^ba[rz]*$'` : 최대 2단계 디렉토리 깊이까지 정규 표현식과 일치하는 파일 찾기

## Links
- More information: https://github.com/monochromegane/the_platinum_searcher
