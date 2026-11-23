# dnf clean

Red Hat 계열 저장소의 임시 파일 정리 수행.

## Usage
- `dnf clean dbcache` : 저장소 메타데이터로 생성된 캐시 파일 제거
- `dnf clean expire-cache` : 저장소 메타데이터를 만료 상태로 표시
- `dnf clean metadata` : 저장소 메타데이터 제거
- `dnf clean packages` : 시스템에 캐시된 패키지 제거
- `dnf clean all` : 모든 DNF 저장소 메타데이터 및 캐시 파일 정리 (위 항목 전체 수행)

## Links
- More information: https://dnf.readthedocs.io/en/latest/command_ref.html#clean-command
