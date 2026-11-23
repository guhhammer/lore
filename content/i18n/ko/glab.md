# glab

GitLab으로 원활하게 작업. `repo`와 같은 일부 하위 명령에는 자체 사용법 문서가 있음.

## Usage
- `glab repo clone 소유자/레포지토리` : 로컬에서 GitLab 저장소를 복제
- `glab issue create` : 새로운 이슈 생성
- `glab issue list` : 현재 저장소의 공개 이슈를 보고 필터링
- `glab issue view -w 이슈_번호` : 기본 브라우저에서 이슈 보기
- `glab mr create` : 병합 요청을 생성
- `glab mr view -w pr_번호` : 기본 웹 브라우저에서 풀 요청 보기
- `glab co pr_번호` : 특정 풀 요청을 로컬에서 확인

## Links
- More information: https://gitlab.com/gitlab-org/cli/-/tree/main/docs/source
