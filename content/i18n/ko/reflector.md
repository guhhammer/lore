# reflector

Arch 스크립트로 미러리스트를 가져오고 정렬합니다.

## Usage
- `sudo reflector --sort rate --save /etc/pacman.d/mirrorlist` : 모든 미러를 가져와 다운로드 속도로 정렬하고 저장
- `reflector --country Germany --protocol https` : 독일의 HTTPS 미러만 가져오기
- `reflector --latest 10` : 최근에 동기화된 10개의 미러만 가져오기

## Links
- More information: https://manned.org/reflector
