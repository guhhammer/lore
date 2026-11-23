# trivy

컨테이너 이미지, 파일 시스템 및 Git 저장소의 취약점과 구성 문제를 스캔하는 도구.

## Usage
- `trivy image 이미지:태그` : docker 이미지를 취약점 및 노출된 비밀 키에 대해 스캔
- `trivy image -s HIGH,CRITICAL alpine:3.15` : 심각도에 따라 출력 결과를 필터링하여 Docker 이미지 스캔
- `trivy image --ignore-unfixed alpine:3.15` : 수정되지 않거나 패치되지 않은 취약점을 무시하고 Docker 이미지 스캔
- `trivy fs --scanners vuln,misconfig 경로/대상/프로젝트_폴더` : 파일 시스템을 취약점 및 잘못된 구성에 대해 스캔
- `trivy config 경로/대상/iac_폴더` : iaC(Terraform, CloudFormation, ARM, Helm 및 Dockerfile) 디렉토리를 잘못된 구성에 대해 스캔
- `trivy repo 경로/대상/로컬_저장소_폴더|원격_저장소_url` : 로컬 또는 원격 Git 저장소를 취약점에 대해 스캔
- `trivy repo --commit 커밋_해시 저장소` : 특정 커밋 해시까지 Git 저장소 스캔
- `trivy image -f template -t "@sarif.tpl" -o 경로/대상/보고서.sarif 이미지:태그` : SARIF 템플릿으로 출력 생성

## Links
- More information: https://trivy.dev/docs/latest/guide/references/configuration/cli/trivy/
