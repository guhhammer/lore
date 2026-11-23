# gh run

최근 GitHub Actions 워크플로 실행을 보고, 실행하고, 모니터링.

## Usage
- `gh run view` : 실행을 인터랙티브하게 선택하여 작업 정보 보기
- `gh run view workflow_run_number` : 특정 실행에 대한 정보 표시
- `gh run view -j job_number` : 작업의 단계에 대한 정보 표시
- `gh run view -j job_number --log` : 작업의 로그 표시
- `gh run view workflow_run_number --exit-status && echo "run pending or passed"` : 특정 워크플로를 확인하고 실행이 실패한 경우 0이 아닌 상태로 종료
- `gh run watch` : 활성 실행을 인터랙티브하게 선택하고 완료될 때까지 대기
- `gh run watch workflow_run_number` : 실행의 작업을 표시하고 완료될 때까지 대기
- `gh run rerun workflow_run_number` : 특정 워크플로 재실행

## Links
- More information: https://cli.github.com/manual/gh_run
