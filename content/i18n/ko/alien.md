# alien

다양한 설치 패키지를 다른 형식으로 변환합니다.

## Usage
- `sudo alien -d 경로/대상/파일` : 특정 설치 파일을 Debian 형식(`.deb` 확장자)으로 변환
- `sudo alien -r 경로/대상/파일` : 특정 설치 파일을 Red Hat 형식(`.rpm` 확장자)으로 변환
- `sudo alien -t 경로/대상/파일` : 특정 설치 파일을 Slackware 설치 파일(`.tgz` 확장자)로 변환
- `sudo alien -d -i 경로/대상/파일` : 특정 설치 파일을 Debian 형식으로 변환하고 시스템에 설치

## Links
- More information: https://manned.org/alien
