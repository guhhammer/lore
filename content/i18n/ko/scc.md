# scc

코드 라인 수를 계산합니다. Go로 작성되었습니다.

## Usage
- `scc` : 현재 디렉토리의 코드 라인 수 출력
- `scc 경로/대상/폴더` : 대상 디렉토리의 코드 라인 수 출력
- `scc --by-file` : 각 파일에 대한 출력 표시
- `scc -f tabular|wide|json|csv|cloc-yaml|html|html-table` : 특정 출력 형식을 사용하여 출력 표시 (기본값은 `tabular`)
- `scc -i go,java,js` : 특정 파일 확장자를 가진 파일만 계산
- `scc --exclude-dir .git,.hg` : 카운트에서 제외할 디렉토리 지정
- `scc -s files|name|lines|blanks|code|comments|complexity` : 출력 및 정렬 기준 열로 정렬 (기본값은 파일 기준)
- `scc -h` : 도움말 표시

## Links
- More information: https://github.com/boyter/scc#usage
