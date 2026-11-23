# conda export

환경 상세 정보 내보내기.

## Usage
- `conda export` : 현재 환경의 상세 정보를 `stdout`으로 출력
- `conda export -f 경로/대상/환경.yaml` : 현재 환경의 상세 정보를 `YAML` 파일로 내보내기
- `conda export --format environment-json|environment-yaml|explicit|json|reqs|requirements|txt|yaml|yml` : 특정 형식으로 상세 정보 내보내기
- `conda export -n 환경_이름` : 이름으로 환경을 지정
- `conda export -p 경로/대상/환경` : 경로로 환경 지정
- `conda export -c 채널_이름` : 특정 채널 포함

## Links
- More information: https://docs.conda.io/projects/conda/en/latest/commands/export.html
