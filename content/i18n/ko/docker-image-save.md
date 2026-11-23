# docker image save

Docker 이미지를 아카이브로 내보내기.

## Usage
- `docker save 이미지:태그 >경로/대상/파일.tar` : 이미지를 `stdout`를 통해 `.tar` 아카이브로 저장
- `docker save -o 경로/대상/파일.tar 이미지:태그` : 이미지를 `.tar` 아카이브로 저장
- `docker save -o 경로/대상/파일.tar 이미지_이름` : 이미지의 모든 태그 저장
- `docker save -o 경로/대상/파일.tar 이미지_이름:태그1 이미지_이름:태그2 ...` : 저장할 이미지의 특정 태그 선택

## Links
- More information: https://docs.docker.com/reference/cli/docker/image/save/
