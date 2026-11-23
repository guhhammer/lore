# jdupes

강력한 중복 파일 찾기 도구이며, fdupes의 개선된 포크입니다.

## Usage
- `jdupes 경로/대상/폴더` : 단일 디렉터리 검색
- `jdupes 디렉터리1 디렉터리2 ...` : 여러 디렉터리 검색
- `jdupes -r 경로/대상/폴더` : 모든 디렉터리를 재귀적으로 검색
- `jdupes -d -r 경로/대상/폴더` : 디렉터리를 재귀적으로 검색하고 사용자가 보관할 파일을 선택하도록 허용
- `jdupes 디렉터리1 -R 디렉터리2` : 여러 디렉터리를 검색하고 디렉터리1이 아닌 디렉터리2의 하위 디렉터리를 따라가며 검색
- `jdupes -O 디렉터리1 디렉터리2 디렉터리3 ...` : 여러 디렉터리를 검색하고 결과에서 디렉터리 순서를 유지

## Links
- More information: https://codeberg.org/jbruchon/jdupes#usage
