# az quantum

Manage Azure Quantum 워크스페이스를 관리하고 양자 작업을 제출하는 명령어(프리뷰, quantum 확장이 필요).

## Usage
- `az quantum workspace create -g 리소스_그룹 -l 위치 -w 워크스페이스 -a 내_스토리지_계정_이름` : 새로운 Azure Quantum 워크스페이스 생성
- `az quantum workspace list` : 모든 Azure Quantum 워크스페이스 목록 조회
- `az quantum workspace set -g 리소스_그룹 -w 워크스페이스` : 기본 Azure Quantum 워크스페이스 설정
- `az quantum job submit -g 리소스_그룹 -w 워크스페이스 -l 위치 -t Id --job-name 작업 --job-input-file QirBitcode.bc --job-input-format qir.v1` : 특정 타겟에 QIR 양자 작업 제출
- `az quantum job list -g 리소스_그룹 -l 위치 -w 워크스페이스` : quantum 워크스페이스 내 모든 작업 목록 조회
- `az quantum job output -g 리소스_그룹 -w 워크스페이스 --job-id 작업` : 양자 작업 결과 출력
- `az quantum offerings list -l Location` : 특정 지역에서 사용 가능한 제공자 목록 조회
- `az quantum target set -t 아이디` : 작업 제출을 위한 기본 타겟 설정

## Links
- More information: https://learn.microsoft.com/cli/azure/quantum
