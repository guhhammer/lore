# trunk.io

코드에 대해 린터(linters), 포맷터(formatters) 및 보안 분석 도구를 실행.

## Usage
- `trunk init` : 저장소에서 trunk를 초기화
- `trunk check` : 변경된 파일에 대해 적용 가능한 모든 린터와 포맷터를 실행
- `trunk check 경로/대상/파일1 경로/대상/파일2 ...` : 특정 파일에 대해 린터와 포맷터를 실행
- `trunk fmt` : 파일을 제자리에서 포맷
- `trunk tools list` : 사용 가능한 모든 도구와 상태를 표시
- `trunk tools enable 도구@버전` : 특정 버전의 도구를 활성화
- `trunk actions history 액션` : 액션의 실행 기록을 출력
