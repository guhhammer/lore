# debsecan

Debian 보안 분석기, 특정 Debian 설치에서 취약점을 나열하는 도구.

## Usage
- `debsecan` : 현재 호스트에서 취약한 설치 패키지 나열
- `debsecan --suite 릴리스_코드_이름` : 특정 스위트의 취약한 설치 패키지 나열
- `debsecan --suite 릴리스_코드_이름 --only-fixed` : 수정된 취약점만 나열
- `debsecan --suite sid --only-fixed --format report --mailto root --update-history` : 불안정("sid") 버전의 수정된 취약점만 나열하고 루트로 메일 발송
- `sudo apt upgrade $(debsecan --only-fixed --format packages)` : 취약한 설치 패키지 업그레이드

## Links
- More information: https://manned.org/debsecan
