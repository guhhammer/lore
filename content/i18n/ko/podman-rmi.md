# podman rmi

OCI/Docker 이미지 제거.

## Usage
- `podman rmi 이미지:태그 이미지2:태그 ...` : 이름을 지정하여 하나 이상의 이미지 제거
- `podman rmi -f 이미지` : 강제로 이미지 제거
- `podman rmi --no-prune 이미지` : 태그되지 않은 부모를 삭제하지 않고 이미지 제거
- `podman rmi -h` : 도움말 표시

## Links
- More information: https://docs.podman.io/en/latest/markdown/podman-rmi.1.html
