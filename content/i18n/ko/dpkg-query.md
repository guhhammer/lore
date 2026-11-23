# dpkg-query

설치된 패키지에 대한 정보 표시.

## Usage
- `dpkg-query -l` : 설치된 모든 패키지 나열
- `dpkg-query -l 'libc6*'` : 패턴과 일치하는 설치된 패키지 나열
- `dpkg-query -L libc6` : 패키지에 의해 설치된 모든 파일 나열
- `dpkg-query -s libc6` : 패키지에 대한 정보 표시
- `dpkg-query -S /etc/ld.so.conf.d` : 패턴과 일치하는 파일을 소유한 패키지 검색

## Links
- More information: https://manned.org/dpkg-query
