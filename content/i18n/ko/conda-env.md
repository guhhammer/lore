# conda env

conda 환경 관리.

## Usage
- `conda env create -f 경로/대상/파일` : 환경 설정 파일(YAML, TXT 등)을 통해 환경 생성
- `conda env remove -n 환경_이름` : 환경 및 내부 모든 항목 삭제
- `conda env update -f 경로/대상/파일 --prune` : 환경 파일을 기반으로 환경을 업데이트
- `conda env list` : 모든 환경 목록 표시
- `conda env export` : 환경 상세 정보 확인
- `conda env config vars list` : 환경 변수 목록 표시
- `conda env config vars set 환경변수_키=값` : 환경 변수 설정

## Links
- More information: https://docs.conda.io/projects/conda/en/latest/commands/env/index.html
