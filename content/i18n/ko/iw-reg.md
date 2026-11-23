# iw reg

WLAN 규제 도메인을 관리.

## Usage
- `iw reg get` : 현재 규제 도메인 표시
- `sudo iw reg set US|JP|FI|...` : ISO 3166-1 alpha-2 국가 코드를 사용하여 규제 도메인 설정
- `sudo iw reg reload` : 커널 규제 데이터베이스 다시 불러오기

## Links
- More information: https://wireless.docs.kernel.org/en/latest/en/users/documentation/iw.html
