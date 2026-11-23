# fd

`find`의 대안.

## Usage
- `fd "regex"` : 현재 디렉터리에서 특정 패턴과 일치하는 파일을 재귀적으로 찾기
- `fd "regex" 경로/대상/디렉터리` : 특정 디렉터리에서 파일 찾기
- `fd -e txt` : 특정 확장자를 가진 파일 찾기
- `fd "regex" -t d` : 특정 패턴과 일치하는 디렉터리만 찾기
- `fd "regex" -HI` : 검색에 무시되거나 숨겨진 파일을 포함
- `fd "regex" -E glob` : 특정 `glob` 패턴과 일치하는 파일을 제외
- `fd "regex" -x 명령어` : 반환된 각 검색 결과에 대해 명령을 실행
- `fd "regex" -d 1` : 현재 디렉터리에서만 파일 찾기

## Links
- More information: https://github.com/sharkdp/fd#command-line-options
