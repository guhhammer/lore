# edquota

사용자 또는 그룹의 쿼터를 편집. 기본적으로 쿼터가 있는 모든 파일 시스템에서 작동합니다. 쿼터 정보는 파일 시스템의 루트에 있는 `quota.user` 및 `quota.group` 파일에 영구적으로 저장됩니다.

## Usage
- `edquota -u $(whoami)` : 현재 사용자의 쿼터 편집
- `sudo edquota -u 사용자명` : 특정 사용자의 쿼터 편집
- `sudo edquota -g 그룹` : 그룹의 쿼터 편집
- `sudo edquota -f 파일_시스템` : 지정된 파일 시스템으로 작업 제한 (기본적으로 edquota는 쿼터가 있는 모든 파일 시스템에서 작동합니다)
- `sudo edquota -t` : 기본 유예 기간 편집
- `sudo edquota -p 참조_사용자 대상_사용자1 대상_사용자2 ...` : 다른 사용자에게 쿼터 복제

## Links
- More information: https://manned.org/edquota
