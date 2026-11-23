# isoinfo

ISO 디스크 이미지 덤프 및 검증 유틸리티 프로그램.

## Usage
- `isoinfo -f -i 경로/대상/이미지.iso` : ISO 이미지에 포함된 모든 파일 나열
- `isoinfo -i 경로/대상/이미지.iso -x /PATH/TO/FILE/INSIDE/ISO.EXT` : ISO 이미지에서 특정 x파일을 추출하여 `stdout`으로 출력
- `isoinfo -d -i 경로/대상/이미지.iso` : ISO 디스크 이미지의 헤더 정보 표시

## Links
- More information: https://manned.org/isoinfo
