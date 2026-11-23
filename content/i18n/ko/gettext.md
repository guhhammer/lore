# gettext

컴파일된 `.mo` 파일에 저장된 번역 문자열을 사용하여 문자열 번역. 번역 파일은 `/usr/share/locale/locale_name/LC_MESSAGES/` 경로에 저장, `domain`은 확장자를 제외한 파일 이름을 의미.

## Usage
- `LANGUAGE=locale gettext -d 도메인 "msgid"` : 도메인 파일에 정의된 문자열 번역 조회 (번역이 없으면 원본 `msgid` 반환)
- `gettext -h` : 도움말 표시
- `gettext -V` : 버전 표시

## Links
- More information: https://www.gnu.org/software/gettext/manual/gettext.html#gettext-Invocation
