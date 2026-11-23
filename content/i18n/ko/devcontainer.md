# devcontainer

Docker 컨테이너를 개발 환경으로 사용.

## Usage
- `devcontainer up` : dev Container 생성 및 실행
- `devcontainer templates apply -t 템플릿_id -a 템플릿_매개변수 -w 경로/대상/작업공간` : 작업 공간에 Dev Container 템플릿을 적용
- `devcontainer exec 명령어` : 현재 작업공간에서 실행 중인 Dev Container에 명령을 실행
- `devcontainer build 경로/대상/작업공간` : `devcontainer.json`에서 Dev Container 이미지를 빌드
- `devcontainer read-configuration` : `devcontainer.json`에서 Dev Container의 구성을 읽고 출력

## Links
- More information: https://containers.dev/implementors/reference/
