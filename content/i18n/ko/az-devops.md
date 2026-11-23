# az devops

Azure DevOps 조직을 관리. `azure-cli`의 일부 (`az`라고도 함).

## Usage
- `az devops login --org 조직_url` : 특정 조직에 로그인하려면 개인 액세스 토큰(PAT)을 설정
- `az devops project show -p 프로젝트_이름 --open` : 브라우저에서 프로젝트를 열기
- `az devops team list-member -p 프로젝트_이름 --team 팀_이름` : 특정 프로젝트에 참여하는 특정 팀의 구성원을 나열
- `az devops configure -l` : azure DevOps CLI 현재 구성을 확인
- `az devops configure -d project=프로젝트_이름 organization=조직_url` : 기본 프로젝트와 기본 조직을 설정하여 Azure DevOps CLI 동작을 구성

## Links
- More information: https://learn.microsoft.com/cli/azure/devops
