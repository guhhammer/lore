# docker image ls

Docker 이미지를 목록으로 출력.

## Usage
- `docker images` : 모든 Docker 이미지 목록 출력
- `docker images -a` : 중간 이미지까지 포함해 Docker 이미지 출력
- `docker images -q` : 간략한 출력 모드 (숫자 ID만 표시)
- `docker images -f dangling=true` : 어떤 컨테이터에서도 사용되지 않는 Docker 이미지 목록 출력
- `docker images "*name*"` : 이름에 특정 문자열이 포함된 이미지 목록 출력
- `docker images --format "\{\{.ID\}\}\t\{\{.Size\}\}\t\{\{.Repository\}\}:\{\{.Tag\}\}" | sort -k 2 -h` : 이미지 크기 기준으로 정렬

## Links
- More information: https://docs.docker.com/reference/cli/docker/image/ls/
