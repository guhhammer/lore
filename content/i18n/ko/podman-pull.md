# podman pull

컨테이너 레지스트리에서 이미지를 가져오기.

## Usage
- `podman pull 이미지:tag` : 지정한 컨테이너 이미지 가져오기
- `podman pull -q 이미지:태그` : 조용한 모드로 컨테이너 이미지 가져오기
- `podman pull -a 이미지` : 컨테이너 이미지의 모든 태그 가져오기
- `podman pull --platform linux/arm64 이미지:tag` : 지정한 플랫폼용 컨테이너 이미지 가져오기
- `podman pull --tls-verify=false 이미지:태그` : TLS 검증 없이 컨테이너 이미지 가져오기
- `podman pull -h` : 도움말 표시

## Links
- More information: https://docs.podman.io/en/latest/markdown/podman-pull.1.html
