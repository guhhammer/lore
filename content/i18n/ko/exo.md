# exo

Exoscale 서비스를 관리하는 도구. `compute`와 같은 일부 하위 명령어는 별도의 사용법 문서를 가짐.

## Usage
- `exo config` : exo 커멘드라인 구성
- `exo completion zsh` : 지정한 쉘에 대한 자동완성 스크립트 생성
- `exo zone -O json` : 사용 가능한 모든 zone을 json 형식으로 출력
- `exo compute instance create 인스턴스_이름 --zone zone -Q` : 특정 zone에서 Compute 인스턴스를 조용히 생성(불필요한 명령어 출력을 비활성화)
- `exo storage list --output-template '\{\{ .Name \}\}'` : 조직의 모든 버킷 이름만 출력
- `exo iam -h` : 특정 하위 명령어 도움말 표시

## Links
- More information: https://community.exoscale.com/reference/cli/exo/storage/list/index.md
