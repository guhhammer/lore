# treemd

트리 기반 탐색과 대화형 TUI를 사용하여 markdown 파일을 확인.

## Usage
- `treemd 경로/대상/파일.md` : 대화형 TUI 모드로 markdown 파일을 확인
- `treemd -l 경로/대상/파일.md` : markdown 파일의 모든 제목 목록 표시
- `treemd --tree 경로/대상/파일.md` : markdown 파일의 제목 트리 구조 표시
- `treemd -s 제목_이름 경로/대상/파일.md` : 제목 이름을 기준으로 특정 섹션 추출
- `treemd -l --filter 패턴 경로/대상/파일.md` : 지정한 패턴으로 제목 필터링
- `treemd -q '.h2 | text' 경로/대상/파일.md` : treemd 쿼리 언어로 markdown 구조를 조회하고 추출

## Links
- More information: https://github.com/Epistates/treemd#usage
