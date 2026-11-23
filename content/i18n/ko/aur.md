# aur

AUR에서 패키지를 빌드하고 로컬 저장소를 관리합니다. 참고: 완전히 사용하려면 `/etc/pacman.conf`에 로컬 저장소가 정의되어 있어야 하며 `vifm`이 설치되어 있어야 합니다.

## Usage
- `repo-add 경로/대상/database.db.tar.gz` : `/etc/pacman.conf`에 설정된 경로와 일치하는 저장소 초기화
- `aur search 키워드` : AUR 데이터베이스에서 패키지 검색
- `aur sync 패키지1 패키지2 ...` : 하나 이상의 패키지와 의존성을 AUR에서 다운로드하고, 빌드하여, 로컬 저장소에 추가
- `aur repo -l` : 로컬 저장소의 패키지 목록 출력
- `aur sync -u` : 로컬 저장소 패키지 업그레이드
- `aur sync -C 패키지` : 설치 후 빌드 파일 정리
- `aur sync --noview -n 패키지` : vim에서 변경 사항 확인 없이, 의존성 설치도 확인 없이 패키지 설치
- `repo-remove 경로/대상/database.db.tar.gz 패키지` : 저장소 메타데이터에서 패키지 제거 (패키지 파일 자체는 삭제되지 않음)

## Links
- More information: https://github.com/aurutils/aurutils
