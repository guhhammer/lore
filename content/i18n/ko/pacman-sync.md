# pacman-sync

Arch Linux 패키지 관리 도구.

## Usage
- `sudo pacman --sync 패키지` : 새 패키지 설치
- `sudo pacman --sync --refresh --sysupgrade` : 모든 패키지 동기화 및 업데이트 (`--downloadonly`를 추가하여 패키지를 다운로드만 하고 업데이트하지 않음)
- `sudo pacman --sync --refresh --sysupgrade --noconfirm 패키지` : 모든 패키지를 업데이트하고 새 패키지를 확인 없이 설치
- `pacman --sync --search "검색어"` : 정규 표현식 또는 키워드로 패키지 데이터베이스 검색
- `pacman --sync --info 패키지` : 패키지 정보 표시
- `sudo pacman --sync --refresh --sysupgrade --overwrite 경로/대상/파일` : 패키지 업데이트 중 충돌하는 파일 덮어쓰기
- `sudo pacman --sync --refresh --sysupgrade --ignore 패키지` : 모든 패키지를 동기화 및 업데이트하지만 특정 패키지는 무시 (여러 번 사용 가능)
- `sudo pacman --sync --clean` : 설치되지 않은 패키지와 사용되지 않는 저장소를 캐시에서 제거 (모든 패키지를 정리하려면 `--clean` 플래그를 두 번 사용)

## Links
- More information: https://manned.org/pacman.8
