# gh agent-task

GitHub 에이전트 작업 관리.

## Usage
- `gh agent list` : 최근 에이전트 작업 목록 표시
- `gh agent create "데이터 처리 파이프라인의 성능을 개선하세요."` : 현재 저장소에 새로운 에이전트 작업 생성
- `gh agent create -F 경로/대상/파일` : 파일로부터 새로운 에이전트 작업 생성
- `gh agent view 세션_아이디|pr_번호|주소|브랜치` : 특정 에이전트 작업 상세 정보 표시
- `gh agent view --log 세션_아이디|pr_번호|주소|브랜치` : 특정 에이전트 작업 로그 표시

## Links
- More information: https://cli.github.com/manual/gh_agent-task
