# podman load

`podman save`로 생성한 oci 아카이브 또는 docker 아카이브에서 이미지를 불러옴.

## Usage
- `podman load -i 경로/대상/파일.tar` : `.tar` 파일에서 이미지 불러오기
- `podman load -i 경로/대상/파일.tar.ext` : 압축된 `.tar` 파일에서 이미지 불러오기
- `podman load -q -i 경로/대상/파일.tar` : 이미지를 불러오고 이미지 ID만 출력
- `podman <경로/대상/파일.tar load` : `stdin`에서 이미지 불러오기

## Links
- More information: https://docs.podman.io/en/latest/markdown/podman-load.1.html
