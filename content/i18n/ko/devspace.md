# devspace

Kubernetes에서 애플리케이션을 개발, 배포 및 디버깅.

## Usage
- `devspace init` : 현재 디렉터리에서 새로운 DevSpace 프로젝트를 초기화
- `devspace dev` : 포트 포워딩, 파일 동기화, 터미널 접근을 포함한 개발 모드를 시작
- `devspace dev -n 네임스페이스` : 특정 네임스페이스에서 개발 모드를 시작
- `devspace deploy` : 프로젝트를 Kubernetes에 배포
- `devspace deploy -p 프로파일_이름` : 특정 프로파일을 사용하여 프로젝트를 배포
- `devspace build` : 정의된 모든 이미지를 빌드
- `devspace logs -f` : pod의 로그를 실시간으로 확인
- `devspace ui` : 브라우저에서 DevSpace UI를 열기

## Links
- More information: https://www.devspace.sh/docs/cli
