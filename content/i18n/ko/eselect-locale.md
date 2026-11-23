# eselect locale

시스템 언어를 설정하는 `$LANG` 환경 변수를 관리하기 위한 `eselect` 모듈.

## Usage
- `eselect locale list` : 사용 가능한 로케일 나열
- `eselect locale set 이름|인덱스` : `list` 명령의 이름이나 인덱스로 `/etc/profile.env`에 `$LANG` 환경 변수 설정
- `eselect locale show` : `/etc/profile.env`의 `$LANG` 값 표시

## Links
- More information: https://wiki.gentoo.org/wiki/Eselect#Locale
