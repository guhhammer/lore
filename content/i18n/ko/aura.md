# aura

Aura 패키지 매니저(Aura Package Manager): Arch Linux 및 AUR를 위한 안전하고 다국어를 지원하는 패키지 관리자.

## Usage
- `aura -As 키워드|정규표현시` : AUR에서 패키지 검색
- `aura -A 패키지` : AUR에서 패키지 설치
- `aura -Akua` : 모든 AUR 패키지를 상세 모드로 업데이트하고 make 의존성을 제거
- `aura -S 패키지` : 공식 저장소에서 패키지 설치
- `aura -Syu` : 공식 저장소의 모든 패키지 동기화 및 업데이트
- `aura -Rsu 패키지` : 패키지 및 의존성 제거
- `aura -Oj` : orphan 패키지 제거 (어떤 패키지도 필요로 하지 않는 의존성 패키지)

## Links
- More information: https://github.com/fosskers/aura
