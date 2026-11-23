# dnf config-manager

Fedora 기반 시스템에서 DNF 구성 옵션과 저장소를 관리합니다.

## Usage
- `dnf config-manager --add-repo=저장소_url` : URL에서 저장소 추가(그리고 활성화)
- `dnf config-manager --dump` : 현재 구성 값 출력
- `dnf config-manager --enable 저장소_id` : 특정 저장소 활성화
- `dnf config-manager --disable 저장소_id1 저장소_id2 ...` : 지정된 저장소 비활성화
- `dnf config-manager --setopt=옵션=값` : 저장소에 대한 구성 옵션 설정
- `dnf config-manager --help-cmd` : 도움말 표시

## Links
- More information: https://dnf-plugins-core.readthedocs.io/en/latest/config_manager.html
