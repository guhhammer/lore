# grype

컨테이너 이미지 및 파일 시스템의 취약점 스캐너.

## Usage
- `grype 이미지:태그` : 컨테이너 이미지 스캔
- `grype 이미지:태그 -o json|table|cyclonedx|cyclonedx-json|sarif|template` : 특정 형식으로 결과 출력하며 이미지 스캔
- `grype 이미지:태그 --only-fixed` : 수정되지 않은 취약점은 제외하고 이미지 스캔
- `grype 이미지:태그 -f negligible|low|medium|high|critical` : 지정한 심각도 이상 취약점 발견 시, 실패로 표시하고 이미지 스캔
- `grype 경로/대상/디렉터리 --file 경로/대상/리포트` : 로컬 디렉터리를 스캔하고 보고서를 파일로 저장
- `grype db update` : 취약점 데이터베이스 업데이트
- `grype db status` : 현재 데이터베이스 상태 표시
- `grype -h` : 도움말 표시

## Links
- More information: https://oss.anchore.com/docs/reference/grype/cli
