# pacman-query

Arch Linux 패키지 관리 도구.

## Usage
- `pacman -Q` : 설치된 패키지와 버전 나열
- `pacman -Qe` : 명시적으로 설치된 패키지와 버전만 나열
- `pacman -Qo 파일_이름` : 파일을 소유한 패키지 찾기
- `pacman -Qi 패키지` : 설치된 패키지 정보 표시
- `pacman -Ql 패키지` : 패키지가 소유한 파일 나열
- `pacman -Qdtq` : 고아 패키지 나열 (의존성으로 설치되었지만 더 이상 어떤 패키지도 필요로 하지 않는 패키지)
- `pacman -Qm` : 저장소에서 찾을 수 없는 설치된 패키지 나열
- `pacman -Qu` : 오래된 패키지 나열

## Links
- More information: https://manned.org/pacman.8
