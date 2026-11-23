# conda remove

conda 환경에서 패키지 제거.

## Usage
- `conda remove scipy` : 현재 활성화된 환경에서 `scipy` 제거
- `conda remove -n 환경_이름 패키지1 패키지2 ...` : 지정한 환경에서 여러 패키지 제거
- `conda remove -n 환경_이름 --all` : 모든 패키지와 환경 자체 제거
- `conda remove -n 환경_이름 --all --keep-env` : 모든 패키지 제거하지만, 환경은 유지

## Links
- More information: https://docs.conda.io/projects/conda/en/latest/commands/remove.html
