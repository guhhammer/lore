# aurman

Arch User Repository에서 패키지를 빌드하고 설치하는 Arch Linux 유틸리티.

## Usage
- `aurman -S -y -u` : 모든 패키지를 동기화하고 업데이트
- `aurman -S -y -u --noedit` : `PKGBUILD` 파일의 변경 사항을 표시하지 않고 모든 패키지를 동기화하고 업데이트
- `aurman -S 패키지` : 새 패키지 설치
- `aurman -S --noedit 패키지` : `PKGBUILD` 파일의 변경 사항을 표시하지 않고 새 패키지 설치
- `aurman -S --noedit --noconfirm 패키지` : 묻지 않고 새 패키지 설치
- `aurman -S -s 키워드` : 공식 저장소와 AUR에서 키워드를 검색하여 패키지 검색
- `aurman --remove --recursive --nosave 패키지` : 특정 패키지 및 의존성 제거
- `aurman -S -c` : 패키지 캐시 지우기 (모든 패키지를 삭제하려면 두 개의 `--clean` 플래그 사용)

## Links
- More information: https://github.com/polygamma/aurman#syntax
