# gh issue

GitHub 이슈 관리.

## Usage
- `gh issue view 이슈_번호` : 특정 이슈 보기
- `gh issue view 이슈_번호 -w` : 기본 웹 브라우저에서 특정 이슈 보기
- `gh issue new -w` : 기본 웹 브라우저에서 새 이슈 생성
- `gh issue ls -L 10 -l "bug"` : `bug` 라벨이 있는 최근 10개의 이슈 나열
- `gh issue ls -s closed -A 사용자_명` : 특정 사용자가 만든 닫힌 이슈 나열
- `gh issue status -R 소유자/저장소` : 특정 저장소의 사용자와 관련된 이슈 상태 표시
- `gh issue reopen 이슈_번호` : 특정 이슈 다시 열기

## Links
- More information: https://cli.github.com/manual/gh_issue
