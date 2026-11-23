# docker image rm

Docker 이미지를 삭제.

## Usage
- `docker rmi 이미지1 이미지2 ...` : 이름으로 하나 이상의 이미지 삭제
- `docker rmi -f 이미지` : 이미지를 강제로 삭제
- `docker rmi --no-prune 이미지` : 태그가 없는 부모 이미지는 삭제하지 않고 이미지 제거
- `docker rmi --help` : 도움말 표시

## Links
- More information: https://docs.docker.com/reference/cli/docker/image/rm/
