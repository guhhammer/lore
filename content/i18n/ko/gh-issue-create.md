# gh issue create

저장소에 GitHub 이슈 생성.

## Usage
- `gh issue new` : 현재 저장소에 대해 대화식으로 새 이슈 생성
- `gh issue new -l "bug"` : `bug` 레이블을 사용하여 대화식으로 새 이슈 생성
- `gh issue new -a user1,user2,...` : 지정된 사용자에게 할당하여 대화식으로 새 이슈 생성
- `gh issue new -t "제목" -b "본문" -a "@me"` : 제목과 본문을 지정하고 현재 사용자에게 할당하여 새 이슈 생성
- `gh issue new -F 경로/대상/파일` : 파일에서 본문 텍스트를 읽어와 대화식으로 새 이슈 생성
- `gh issue new -w` : 기본 웹 브라우저에서 새 이슈 생성
- `gh issue new -h` : 도움말 표시

## Links
- More information: https://cli.github.com/manual/gh_issue_create
