# bun create

템플릿을 사용해 새로운 프로젝트를 생성.

## Usage
- `bun c 템플릿` : 공식 템플릿을 사용해 대화형으로 새 프로젝트를 생성
- `bun c 템플릿 경로/대상/목적지` : 공식 템플릿을 사용해 새로운 디렉터리에 새 프로젝트를 생성
- `bun c https://github.com/사용자이름/레포지토리 경로/대상/목적지` : gitHub 레포지토리 템플릿을 사용해 새로운 프로젝트 생성
- `bun c 경로/대상/템플릿 경로/대상/목적지` : 로컬 템플릿을 사용해 새로운 프로젝트 생성
- `bun c 템플릿 경로/대상/목적지 --force` : 기존 디렉터리가 있을 경우, 덮어쓰고 프로젝트를 생성
- `bun c 템플릿 경로/대상/목적지 --no-git` : git 저장소를 자동으로 초기화하지 않고, 새로운 프로젝트를 생성
- `bun c 템플릿 경로/대상/목적지 --no-install` : 의존성을 자동으로 설치하지 않고 새로운 프로젝트를 생성

## Links
- More information: https://bun.com/docs/runtime/templating/create
