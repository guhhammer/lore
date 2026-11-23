# conda compare

conda 환경 간 패키지 비교.

## Usage
- `conda compare file.yml` : 현재 디렉터리의 패키지와 `file.yml` 파일 패키지 비교
- `conda compare -n myenv 경로/대상/파일.yml` : `myenv` 환경의 패키지와 `file.yml` 파일의 패키지 비교
- `conda compare -p 경로/대상/myenv 경로/대상/파일.yml` : 사용자 지정 경로(prefix)에 있는 `myenv` 환경의 패키지와 `file.yml` 파일의 패키지 비교
- `conda compare -h` : 도움말 표시

## Links
- More information: https://docs.conda.io/projects/conda/en/stable/commands/compare.html
