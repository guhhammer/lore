# docker image load

파일 또는 `stdin`에서 Docker 이미지를 로드합니다.

## Usage
- `docker <경로/대상/이미지_파일.tar load` : `stdin`에서 Docker 이미지 로드
- `docker load -i 경로/대상/이미지_파일.tar` : 특정 파일에서 Docker 이미지 로드
- `docker load -q -i 경로/대상/이미지_파일.tar` : 조용한 모드로 특정 파일에서 Docker 이미지 로드

## Links
- More information: https://docs.docker.com/reference/cli/docker/image/load/
