# apptainer cache

로컬 Apptainer 캐시를 관리하는 명령어.

## Usage
- `apptainer cache list` : 캐시된 모든 컨테이너 이미지 목록 출력
- `apptainer cache list -v` : 상세 정보와 함께 캐시 목록 출력
- `apptainer cache list -T library|oci|shub|blob|...` : 특정 캐시 유형만 출력
- `apptainer cache clean` : 전체 캐시 삭제
- `apptainer cache clean -T library|oci|shub|blob|...` : 특정 캐시 유형만 삭제
- `apptainer cache clean -D 일수` : 지정한 일수보다 오래된 캐시 항목 삭제
- `apptainer cache clean -n` : 실제 삭제 없이 삭제 대상 미리보기
- `apptainer cache clean -f` : 확인 없이 강제로 캐시 삭제

## Links
- More information: https://apptainer.org/docs/user/main/cli/apptainer_cache.html
