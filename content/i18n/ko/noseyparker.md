# noseyparker

텍스트와 Git 히스토리에서 비밀정보 및 민감한 데이터를 탐지하는 도구. 참고: 각 스캔마다 별도의 데이터 저장소 디렉터리 (`--datastore`)를 사용하는 것이 권장됨.

## Usage
- `noseyparker scan 경로/대상/파일_또는_디렉터리 -d 경로/대상/datastore.np` : 로컬 파일 또는 디렉터리에서 비밀 정보 스캔
- `noseyparker report -d 경로/대상/datastore.np` : 이전 스캔 결과 리포트 출력
- `noseyparker report -d 경로/대상/datastore.np -f human|json|jsonl|sarif` : 다른 형식으로 리포트 출력 (기본값은 `human`)
- `noseyparker scan --git-url url -d 경로/대상/datastore.np` : 원격 Git 저장소 (Git 히스토리 포함)에서 비밀 정보 스캔
- `noseyparker scan --github-user|--github-organization 사용자명_또는_조직_이름 -d 경로/대상/datastore.np` : gitHub 사용자 또는 조직의 모든 저장소에서 비밀 정보 스캔
- `noseyparker rules list` : 모든 스캔 규칙 목록 출력
- `noseyparker github repos list --user|--organization 사용자명_또는_조직_이름` : gitHub 사용자 또는 조직의 모든 저장소 목록 출력

## Links
- More information: https://github.com/praetorian-inc/noseyparker#usage-examples
