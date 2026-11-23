# pacman-mirrors

Manjaro Linux용 `pacman` 미러 리스트 생성. `pacman-mirrors`를 실행할 때마다 데이터베이스를 동기화하고 `sudo pacman -Syyu`를 사용하여 시스템을 업데이트해야 합니다.

## Usage
- `sudo pacman-mirrors --fasttrack` : 기본 설정을 사용하여 미러 리스트 생성
- `pacman-mirrors --status` : 현재 미러 상태 확인
- `pacman-mirrors --get-branch` : 현재 브랜치 표시
- `sudo pacman-mirrors --api --set-branch stable|unstable|testing` : 다른 브랜치로 전환
- `sudo pacman-mirrors --geoip` : 거주 국가의 미러만 사용하여 미러 리스트 생성

## Links
- More information: https://wiki.manjaro.org/index.php?title=Pacman-mirrors
