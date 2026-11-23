# sed

스크립트 가능한 방식으로 텍스트 편집.

## Usage
- `명령어 | sed 's/apple/mango/g'` : 모든 입력 줄에서 모든 `apple`(기본 `regex`)항목을 `mango`(기본 `regex`)로 바꾸고 결과를 `stdout`에 출력
- `명령어 | sed -f 경로/대상/script.sed` : 특정 스크립트 파일(file)을 실행하고 결과를 `stdout`에 출력
- `명령어 | sed -n '1p'` : `stdout`에 첫 번째 줄만 출력

## Links
- More information: https://manned.org/sed.1posix
