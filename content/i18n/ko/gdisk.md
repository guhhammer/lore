# gdisk

GPT (GUID Partition Table) 디스크 파티션 관리 도구.

## Usage
- `sudo gdisk -l` : 파티션 목록 표시
- `sudo gdisk /dev/sdX` : 대화형 파티션 관리 도구 시작
- `<?>` : 도움말 메뉴 열기
- `<p>` : 파티션(partition) 테이블 출력
- `<n>` : 새로운(new) 파티션 추가
- `<d>` : 삭제할(delete) 파티션 선택
- `<w>` : 파티션 테이블을 디스크에 저장(write)하고 종료
- `<q>` : 변경 사항을 저장하지 않고 종료(quit)

## Links
- More information: https://manned.org/gdisk
