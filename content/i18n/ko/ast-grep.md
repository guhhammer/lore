# ast-grep

AST 패턴을 사용해 코드를 검색, 린트 및 재작성.

## Usage
- `ast-grep run -p '패턴' 경로/대상/파일` : 파일에서 특정 패턴 검색
- `ast-grep run -p '패턴' -l python 경로/대상/디렉터리` : 특정 언어에서 패턴 검색
- `ast-grep run -p '패턴' -r '대체문자열' 경로/대상/파일` : 패턴과 일치하는 코드 재작성
- `ast-grep scan -r 경로/대상/규칙.yml 경로/대상/디렉터리` : 설정 파일의 규칙 실행
- `ast-grep run -p '패턴' -i 경로/대상/디렉터리` : 대화형으로 코드 검색 및 재작성
- `ast-grep run -h` : 하위 명령어의 도움말 표시

## Links
- More information: https://ast-grep.github.io/reference/cli.html
