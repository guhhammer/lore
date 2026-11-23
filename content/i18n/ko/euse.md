# euse

Gentoo USE 플래그를 활성화, 비활성화하거나 정보를 조회.

## Usage
- `euse -a -g` : 활성화된 전역 USE 플래그 목록 표시
- `euse -a -l` : 활성화된 로컬 USE 플래그 목록 표시
- `sudo euse -E use_플래그` : 전역 USE 플래그 활성화
- `sudo euse -D use_플래그` : 전역 USE 플래그 비활성화 (USE 플래그 이름 앞에 '-'를 추가)
- `sudo euse -P use_플래그` : 전역 USE 플래그 제거

## Links
- More information: https://wiki.gentoo.org/wiki/Euse
