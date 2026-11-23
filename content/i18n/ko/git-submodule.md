# git submodule

서브모듈을 검사하고 업데이트하며 관리합니다.

## Usage
- `git submodule update --init --recursive` : 저장소의 지정된 서브모듈 설치
- `git submodule add repository_url` : git 저장소를 서브모듈로 추가
- `git submodule add repository_url 경로/대상/폴더` : git 저장소를 지정된 폴더에 서브모듈로 추가
- `git submodule foreach git pull` : 모든 서브모듈을 최신 커밋으로 업데이트

## Links
- More information: https://git-scm.com/docs/git-submodule
