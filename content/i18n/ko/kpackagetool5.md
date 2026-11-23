# kpackagetool5

K패키지 관리자: Plasma 패키지 설치, 나열, 제거.

## Usage
- `kpackagetool5 --list-types` : 설치 가능한 모든 패키지 유형 나열
- `kpackagetool5 -t 패키지_유형 -i 경로/대상/폴더` : 디렉토리에서 패키지 설치
- `kpackagetool5 -t 패키지_유형 -u 경로/대상/폴더` : 디렉토리에서 설치된 패키지 업데이트
- `kpackagetool5 -t Plasma/Applet -l -g` : 설치된 플라스모이드 나열 (--global로 모든 사용자에 대해 표시)
- `kpackagetool5 -t Plasma/Applet -r "이름"` : 이름으로 플라스모이드 제거

## Links
- More information: https://manned.org/kpackagetool5
