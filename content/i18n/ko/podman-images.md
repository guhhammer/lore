# podman images

Podman 컨테이너 이미지 관리.

## Usage
- `podman images` : 모든 컨테이너 이미지 나열
- `podman images -a` : 중간 이미지를 포함한 모든 컨테이너 이미지 나열
- `podman images -q` : 조용한 모드로 출력 나열 (숫자 ID만 표시)
- `podman images -f dangling=true` : 어떤 컨테이너에서도 사용되지 않는 모든 컨테이너 이미지 나열
- `podman images "*이미지|이미지*"` : 이름에 특정 부분 문자열이 포함된 이미지 나열

## Links
- More information: https://docs.podman.io/en/latest/markdown/podman-images.1.html
