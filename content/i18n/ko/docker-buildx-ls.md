# docker buildx ls

빌더 인스턴스와 연결된 노드 목록을 출력.

## Usage
- `docker buildx ls` : 빌더 인스턴스 목록 출력
- `docker buildx ls --format "\{\{.NAME\}\}: \{\{.DriverEndpoint\}\}"` : go 템플릿으로 출력 형식 지정

## Links
- More information: https://docs.docker.com/reference/cli/docker/buildx/ls/
