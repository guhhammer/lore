# syft

컨테이너 이미지와 파일 시스템에서 SBOM(Software Bill of Materials)을 생성.

## Usage
- `syft image:tag` : 컨테이너 이미지에서 SBOM 생성
- `syft 경로/대상/디렉터리` : 로컬 디렉터리에서 SBOM 생성
- `syft 경로/대상/아카이브.tar` : 컨테이너 아카이브 파일에서 SBOM 생성
- `syft 이미지:태그 -o spdx-json` : SPDX JSON 형식으로 SBOM 생성
- `syft 이미지:태그 -o cyclonedx-json` : cycloneDX JSON 형식으로 SBOM 생성
- `syft 이미지:태그 -o 형식=경로/대상/출력_파일` : 지정한 형식으로 SBOM을 파일에 저장
- `syft -h` : 도움말 표시
- `syft --version` : 버전 표시

## Links
- More information: https://oss.anchore.com/docs/reference/syft/cli/
