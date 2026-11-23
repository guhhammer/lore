# jira sprint

Jira 프로젝트 보드의 스프린트 관리.

## Usage
- `jira sprint ls` : 탐색기 보기에서 스프린트와 해당 이슈 목록 표시
- `jira sprint ls --current` : 현재 스프린트의 이슈 목록 표시
- `jira sprint ls --current -a $(jira me)` : 현재 스프린트에서 나에게 할당된 이슈 목록 표시
- `jira sprint ls --current -a $(jira me) -y High` : 현재 스프린트에서 나에게 할당된 높은 우선순위의 이슈 목록 표시
- `jira sprint add` : 대화형 프롬프트를 사용하여 스프린트에 이슈 추가

## Links
- More information: https://github.com/ankitpokhrel/jira-cli#sprint
