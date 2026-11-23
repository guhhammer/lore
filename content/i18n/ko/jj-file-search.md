# jj file search

`jj` 저장소 파일에서 내용을 검색.

## Usage
- `jj file search -p "regex"` : 작업 복사본에서 정규 표현식(`regex`)을 포함하는 파일 검색
- `jj file search -p "glob:*pattern*"` : `glob` 패턴과 일치하는 내용을 포함하는 파일 검색
- `jj file search -r revision -p "패턴"` : 지정한 리비전 파일에서 내용 검색
- `jj file search -p "패턴" 경로/대상/파일_또는_디렉터리` : 지정한 경로 또는 파일에서만 내용 검색

## Links
- More information: https://docs.jj-vcs.dev/latest/cli-reference/#jj-file-search
