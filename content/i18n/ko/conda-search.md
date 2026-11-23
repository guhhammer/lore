# conda search

패키지를 검색하고 상세 정보 표시.

## Usage
- `conda search 패키지_이름` : 특정 패키지 검색
- `conda search 패키지_이름 -i` : 패키지와 상세 정보 함께 검색
- `conda search "*string*"` : 패키지 이름에 `string`이 포함된 패키지 검색
- `conda search "패키지_이름>=패키지_버전"` : 특정 버전 이상의 패키지 검색
- `conda search 채널::패키지_이름` : 특정 채널에서 패키지 검색
- `conda search --envs 패키지_이름` : 로컬 환경에 패키지가 설치되어 있는지 검색

## Links
- More information: https://docs.conda.io/projects/conda/en/latest/commands/search.html
