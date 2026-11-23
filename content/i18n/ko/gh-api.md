# gh api

GitHub API에 인증된 HTTP 요청을 보내고 응답을 출력.

## Usage
- `gh api repos/:owner/:repo/releases` : 현재 저장소의 릴리스를 JSON 형식으로 표시
- `gh api -H Accept:application/vnd.github.squirrel-girl-preview+json -f 'content=+1' repos/:owner/:repo/issues/123/reactions` : 특정 이슈에 대해 반응 생성
- `gh api graphql -F name=':repo' -f 'query'` : graphQL 쿼리 결과를 JSON 형식으로 표시
- `gh api -X POST endpoint` : 사용자 지정 HTTP 메서드를 사용하여 요청 전송
- `gh api -i endpoint` : HTTP 응답 헤더를 출력에 포함
- `gh api --silent endpoint` : 응답 본문을 출력하지 않음
- `gh api --hostname github.example.com endpoint` : 특정 GitHub Enterprise 서버에 요청 전송
- `gh api --help` : 하위 명령 도움말 표시

## Links
- More information: https://cli.github.com/manual/gh_api
