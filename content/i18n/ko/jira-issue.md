# jira issue

Jira 프로젝트의 이슈를 관리.

## Usage
- `jira issue ls` : 최근 이슈 목록 표시
- `jira issue ls -a "이메일_또는_표시_이름"` : 지정한 사용자에게 할당된 이슈 목록 표시
- `jira issue ls -a $(jira me) -y High` : 나에게 할당된 높은 우선순위의 이슈 목록 표시
- `jira issue create` : 대화형 프롬프트를 사용하여 이슈 생성
- `jira issue edit` : 대화형 프롬프트를 사용하여 이슈 수정
- `jira issue asg` : 대화형 프롬프트를 사용하여 사용자에게 이슈 할당
- `jira issue mv 이슈_아이디 "In Progress"` : 이슈를 지정한 상태로 변경
- `jira issue view 이슈_아이디` : 터미널에서 `less`를 사용하여 이슈 보기

## Links
- More information: https://github.com/ankitpokhrel/jira-cli#issue
