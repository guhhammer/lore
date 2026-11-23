# apt-file

아직 설치되지 않은 패키지를 포함해, `apt` 패키지에 포함된 파일을 검색.

## Usage
- `sudo apt update` : 메타데이터 데이터베이스를 업데이트
- `apt-file find 경로/대상/파일` : 지정한 파일 또는 경로를 포함하는 패키지를 검색
- `apt-file list 패키지` : 특정 패키지에 포함된 파일 목록을 표시
- `apt-file find -x regex` : `regex`와 일치하는 패키지를 검색

## Links
- More information: https://manned.org/apt-file
