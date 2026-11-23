# repquota

파일 시스템의 기존 파일 쿼터 요약 정보를 표시합니다.

## Usage
- `sudo repquota -a` : 사용 중인 모든 쿼터의 통계 보고
- `sudo repquota -v 파일시스템` : 할당량을 사용하지 않는 사용자도 포함하여 모든 사용자의 쿼터 통계 보고
- `repquota -u 파일시스템` : 사용자에 대한 쿼터 보고
- `sudo repquota -g 파일시스템` : 그룹에 대한 쿼터 보고
- `sudo repquota -s 파일시스템` : 사람이 읽기 쉬운 형식으로 사용된 쿼터 및 제한 보고
- `sudo repquota -augs` : 사람이 읽기 쉬운 형식으로 사용자 및 그룹의 모든 쿼터 보고

## Links
- More information: https://manned.org/repquota
