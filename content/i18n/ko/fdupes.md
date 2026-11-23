# fdupes

일련의 디렉터리에서 중복 파일을 찾음.

## Usage
- `fdupes 경로/대상/디렉터리` : 단일 디렉터리 검색
- `fdupes 디렉터리1 디렉터리2 ...` : 여러 디렉터리 검색
- `fdupes -r 경로/대상/디렉터리` : 재귀적으로 디렉터리를 검색
- `fdupes 디렉터리1 -R 디렉터리2` : 여러 디렉터리를 하나의 재귀적으로 검색
- `fdupes -rH 경로/대상/디렉터리` : 하드링크를 중복으로 간주하여 재귀적으로 검색
- `fdupes -rd 경로/대상/디렉터리` : 반복적으로 중복 항목을 검색하고 유지할 항목을 선택하고, 나머지 항목을 삭제하는 것
- `fdupes -rdN 경로/대상/디렉터리` : 메시지를 표시하지 않고 반복적으로 검색하고 중복 항목을 삭제

## Links
- More information: https://github.com/adrianlopezroche/fdupes#introduction
