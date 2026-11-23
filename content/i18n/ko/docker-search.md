# docker search

Docker Hub에서 Docker 이미지를 검색.

## Usage
- `docker search 키워드` : 이름 또는 키워드로 Docker 이미지 검색
- `docker search -f is-official=true 키워드` : 공식 이미지만 필터링하여 검색
- `docker search -f is-automated=true 키워드` : 자동 빌드 이미지만 필터링하여 검색
- `docker search -f stars=number 키워드` : 최소 별점 기준으로 이미지 검색
- `docker search --limit number 키워드` : 검색 결과 개수 제한
- `docker search -f "\{\{.Name\}\}: \{\{.Description\}\}" 키워드` : 출력 형식 커스터마이징

## Links
- More information: https://docs.docker.com/reference/cli/docker/search/
