# zizmor

GitHub Actions 워크플로우와 액션 정의에서 보안 취약점 탐지 도구.

## Usage
- `zizmor .` : 현재 디렉터리의 모든 워크플로우 및 액션 검사
- `zizmor 경로/대상/워크플로우.yml` : 특정 워크플로우 파일 검사
- `zizmor --gh-token 토큰 사용자명/저장소` : 원격 GitHub 저장소 검사 (GitHub 토큰 필요)
- `zizmor -o 경로/대상/워크플로우.yml` : 오프라인 검사만 수행 (네트워크 요청 없음)
- `zizmor --format sarif 경로/대상/워크플로우.yml` : SARIF 형식으로 결과 출력
- `zizmor --min-severity informational|low|medium|high .` : 최소 심각도 기준으로 결과 필터링
- `zizmor -h` : 도움말 표시
- `zizmor -v` : 버전 표시

## Links
- More information: https://docs.zizmor.sh/quickstart/
